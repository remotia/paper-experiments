FROM remotia:base

COPY ./scripts/analytics/auto_encoding/ ./scripts/analytics/auto_encoding/

RUN apt-get install -y wget imagemagick ffmpeg
RUN wget https://github.com/Netflix/vmaf/releases/download/v2.3.1/vmaf
RUN chmod 700 ./vmaf
RUN mv vmaf /bin/vmaf

ENTRYPOINT [ "vmaf" ]
