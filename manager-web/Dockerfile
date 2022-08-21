FROM node:16.17 AS build

WORKDIR app

COPY ./manager-web/package.json ./
COPY ./manager-web/package-lock.json ./
RUN npm install
COPY ./manager-web ./
RUN npm run build

FROM nginx:1.23.1-alpine
COPY ./manager-web/dist /usr/share/nginx/html