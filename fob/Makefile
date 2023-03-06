fob: FORCE
	head -c 32 /dev/random > ${SECRETS_DIR}/SECRET_SEED
	head -c 32 /dev/random > ${SECRETS_DIR}/PAIRING_PRIVATE_KEY
# Create the verifying/public key and EEPROM files, since the EEPROM file has a fixed size and the verifying/public key is shorter than the field bounds.
	head -c 2048 /dev/zero > ${EEPROM_PATH}
	head -c 128 /dev/zero > ${SECRETS_DIR}/PAIRING_VERIFYING_KEY
	head -c 128 /dev/zero > ${SECRETS_DIR}/PAIRING_PUBLIC_KEY
	head -c 128 /dev/zero > ${SECRETS_DIR}/FEATURE_VERIFYING_KEY
	rm -f .cargo
	mv cargo/ .cargo/ 2> /dev/null || true
	cargo build --bin ucsc-ectf-fob --release --out-dir /tmp/bld -Z unstable-options
	cp /tmp/bld/ucsc-ectf-fob ${ELF_PATH}
	arm-none-eabi-objcopy -O binary /tmp/bld/ucsc-ectf-fob ${BIN_PATH}

paired_fob: fob
unpaired_fob: fob

FORCE:;
