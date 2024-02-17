FROM openjdk:17-alpine

RUN mkdir -p /gatling

ENV GATLING_VERSION 3.10.3

RUN apk add --update wget bash libc6-compat && \
  mkdir -p /tmp/downloads && \
  wget -q -O /tmp/downloads/gatling-$GATLING_VERSION.zip \
  https://repo1.maven.org/maven2/io/gatling/highcharts/gatling-charts-highcharts-bundle/$GATLING_VERSION/gatling-charts-highcharts-bundle-$GATLING_VERSION-bundle.zip && \
  mkdir -p /tmp/archive && cd /tmp/archive && \
  unzip /tmp/downloads/gatling-$GATLING_VERSION.zip && \
  mv /tmp/archive/gatling-charts-highcharts-bundle-$GATLING_VERSION/* /gatling/ && \
  rm -rf /tmp/*

ADD files /gatling/user-files

WORKDIR /gatling

VOLUME ["/gatling/results"]

ENV PATH /gatling/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
ENV GATLING_HOME /gatling

ENTRYPOINT ["gatling.sh"]