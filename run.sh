#!/bin/sh

genavro(){
	export ENV_SCHEMA_FILENAME=./sample.d/sample.avsc
	cat sample.d/sample.jsonl |
		json2avrows |
		cat > sample.d/sample.avro
}

#genavro

run_native(){
	export ENV_AVRO_CODEC=null
	export ENV_AVRO_CODEC=deflate
	export ENV_AVRO_CODEC=snappy
	export ENV_AVRO_CODEC=bzip2
	export ENV_AVRO_CODEC=xz
	export ENV_AVRO_CODEC=zstandard
	
	cat sample.d/sample.avro |
		./rs-avro-transcode |
		rs-avro2jsons |
		jq -c
}

run_wazero(){
	ENV_AVRO_CODEC=deflate
	ENV_AVRO_CODEC=snappy
	ENV_AVRO_CODEC=null
	
	cat sample.d/sample.avro |
		wazero \
			run \
			-env ENV_AVRO_CODEC="${ENV_AVRO_CODEC}" \
			./rs-avro-transcode.wasm |
		rs-avro2jsons |
		jq -c
}

echo native
run_native

which wazero | fgrep -q wazero || exec sh -c 'echo wazero missing.; exit 1'
echo
echo wazero
run_wazero
