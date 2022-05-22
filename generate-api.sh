rm -rf _yaml ./habfoo_ts_generated ./habfoo-api-generated ./habfoo_client_generated
docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate \
    -i /local/habfoo-api/api/root.yaml \
    -g graphql-schema \
    -o /local/_yaml/

docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate \
    -i /local/habfoo-api/api/root.yaml \
    -g rust-server \
    -o /local/habfoo-api-generated \
    --additional-properties=packageName=habfoo-api-generated

java -jar ~/Downloads/openapi-generator-cli-5.3.1.jar generate \
    -i habfoo-api/api/root.yaml \
    -g dart-dio-next \
    -o ./habfoo_client_generated \
    --additional-properties=pubName=habfoo_client_generated

cd ./habfoo_client_generated
flutter pub run build_runner build
cd ..

docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate \
    -i /local/habfoo-api/api/root.yaml \
    -g typescript \
    -o /local/habfoo_ts_generated 


