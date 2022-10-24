FROM postgres:latest
ENV POSTGRES_PASSWORD=postgres
ENV POSTGRES_USER=postgres
ENV POSTGRES_DB=social_network
RUN apt-get update && apt-get install curl -y libpq-dev -y build-essential -y libssl-dev -y pkg-config -y libpq-dev -y python3 -y python3-pip
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN pip3 install psycopg2 numpy pandas
COPY ./backend /backend
WORKDIR /backend