set -eu
rm -rf _yaml ./habfoo_ts_generated ./habfoo-api-generated ./habfoo_client_generated

#java -jar ~/projects/openapi-generator-cli-5.3.1.jar generate \
   #-i habfoo-api/api/root.yaml \
   #-g graphql-schema \
   #-o ./_yaml/

java -jar ~/projects/openapi-generator-cli-5.3.1.jar generate \
   -i habfoo-api/api/root.yaml \
   -g rust-server \
   -o ./habfoo-api-generated \
   --additional-properties=packageName=habfoo-api-generated

#java -jar ~/projects/openapi-generator-cli-5.3.1.jar generate \
   #-i habfoo-api/api/root.yaml \
   #-g dart-dio-next \
   #-o ./habfoo_client_generated \
   #--additional-properties=pubName=habfoo_client_generated

#cd ./habfoo_client_generated
#flutter pub get
#flutter pub run build_runner build
#cd ..

java -jar ~/projects/openapi-generator-cli-5.3.1.jar generate \
    -i habfoo-api/api/root.yaml \
    -g typescript-fetch \
    -o ./habfoo_ts_generated \
    --additional-properties=npmName=habfoo-api,platform=browser,useObjectParameters=true,typescriptThreePlus=true

cd ./habfoo_ts_generated
npm i
cd ..


