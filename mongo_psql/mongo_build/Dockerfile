FROM mongo

COPY init.json /init.json
CMD mongoimport --host mongodb --db books --collection book_collection --type json --file /init.json --jsonArray
