# ── Stage 1: Build fat JAR ────────────────────────────────────────────────────
FROM eclipse-temurin:21-jdk-alpine AS builder

WORKDIR /build

# Cache Gradle dependencies before copying source
COPY build.gradle.kts settings.gradle.kts gradle.properties ./
COPY gradle/ gradle/
COPY gradlew ./
RUN chmod +x gradlew && ./gradlew dependencies --no-daemon -q 2>/dev/null || true

# Copy source and build
COPY src/ src/
COPY tailwind.config.js package.json ./

# Build Tailwind CSS output (requires Node.js in builder stage)
RUN apk add --no-cache nodejs npm \
 && npm ci \
 && npm run build:css

# Build the fat JAR
RUN ./gradlew shadowJar --no-daemon -q

# ── Stage 2: Runtime ─────────────────────────────────────────────────────────
FROM eclipse-temurin:21-jre-alpine

WORKDIR /app

COPY --from=builder /build/build/libs/*-fat.jar app.jar
COPY --from=builder /build/src/main/resources/static/ src/main/resources/static/

# JTE precompiled classes (if using precompiled mode)
COPY --from=builder /build/jte-classes/ jte-classes/

ENV JVM_OPTS="-Xms256m -Xmx512m -XX:+UseG1GC -XX:MaxGCPauseMillis=100"

EXPOSE 8080

ENTRYPOINT ["sh", "-c", "exec java $JVM_OPTS -jar app.jar"]
