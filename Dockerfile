FROM postgres:latest
ENV POSTGRES_PASSWORD=postgres
ENV POSTGRES_USER=postgres
ENV POSTGRES_DB=social_network
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN apt-get update && apt-get install -y libpq-dev
RUN apt-get install -y build-essential
RUN apt-get install -y libssl-dev
RUN apt-get install -y pkg-config
RUN apt-get install -y libpq-dev
RUN apt-get install -y python3
RUN apt-get install -y python3-pip
RUN pip3 install psycopg2 numpy pandas
COPY ./backend /backend
WORKDIR /backend