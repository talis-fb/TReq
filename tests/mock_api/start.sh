# Dockerfile was generated and saved by mockoon-cli
# Run the following commands to build the image and run the container:
docker build -t mockapi-image .
docker run -d -p 7777:7777 --name mockapi mockapi-image
