# Stage 1: Build
FROM node:20.15.1-alpine AS builder

WORKDIR /app

# Copy package files
COPY sveltekit/package*.json .

# Install dependencies
RUN npm install

# Copy all files from sveltekit directory
COPY sveltekit .

# Build the project
RUN npm run build

# Remove dev dependencies
RUN npm prune --production

# Stage 2: Deploy
FROM node:20.15.1-alpine AS deployer

WORKDIR /app

# Copy build files and package.json from builder stage
COPY --from=builder /app/build build/
COPY --from=builder /app/package.json .

# Expose port
EXPOSE 5090

# Set environment to production
ENV NODE_ENV=production
ENV PORT=5090

# Command to run the application
CMD ["node", "build"]
