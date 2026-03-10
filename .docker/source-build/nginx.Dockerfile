FROM nginx:1.27-alpine

COPY .docker/source-build/nginx.conf /etc/nginx/conf.d/default.conf
