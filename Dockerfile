FROM python:3.9

# set up requirements
ADD ./requirements.txt .
RUN pip install -r ./requirements.txt

# Copy in files
ADD ./config ./config
ADD ./icees_db ./icees_db
ADD ./initdb.py ./initdb.py
ADD ./initdb.sh ./initdb.sh

# set up base for command
ENTRYPOINT ["./initdb.sh"]
