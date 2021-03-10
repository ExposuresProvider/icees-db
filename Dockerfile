FROM python:3.9

# set up requirements
ADD ./requirements.txt .
RUN pip install -r ./requirements.txt

# Copy in files
ADD ./icees_db ./icees_db
ADD ./bin ./bin

# set up base for command
ENTRYPOINT ["./bin/initdb.sh"]
