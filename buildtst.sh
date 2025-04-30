#!/bin/bash
#
# Localized Continuous Integration (CI) for Linux
# 
# This just runs some basic build and format checks on the crate using 
# both the MSRV compiler and the stable version.
#
# This requires the MSRV, nightly, and stable versions of the compiler 
# installed on the local host.
#

# Extract MSRV from Cargo.toml and strip off the quotes
MSRV=$(awk '/rust-version/ { print substr($3, 2, length($3)-2) }' Cargo.toml)
N_DOT=$(echo ${MSRV} | grep -o "\." | wc -l)
[[ ${N_DOT} == 1 ]] && MSRV=${MSRV}.0

printf "Using MSRV ${MSRV}\n\n"

printf "Cleaning the crate...\n"
cargo clean
[[ "$?" != 0 ]] && exit 1
printf "    Ok\n"

printf "\nFormat check...\n"
cargo +nightly fmt --check
[[ "$?" != 0 ]] && exit 1
printf "    Ok\n"

FEATURES="utils serde"

for VER in ${MSRV} stable ; do
    printf "\n\nChecking default features for version: ${VER}...\n"
    cargo clean && \
	cargo +${VER} check && \
	cargo +${VER} doc --all-features --no-deps && \
	cargo +${VER} test
    [[ "$?" != 0 ]] && exit 1
    printf "    Ok\n"

    printf "\n\nChecking no default features for version: ${VER}...\n"
    cargo clean && \
	cargo +${VER} check --no-default-features && \
	cargo +${VER} test --no-default-features
    [[ "$?" != 0 ]] && exit 1
    printf "    Ok\n"

    for FEATURE in ${FEATURES} ; do
        printf "\n\nBuilding with feature [%s] for version %s...\n" "${FEATURE}" "${VER}"
        cargo clean && \
            cargo +"${VER}" check --no-default-features --features="${FEATURE}" && \
            cargo +"${VER}" test --no-default-features --features="${FEATURE}"
        [[ "$?" != 0 ]] && exit 1
    done
done

printf "\nChecking clippy for version: %s...\n" "${MSRV}"
cargo clean
! cargo +"${MSRV}" clippy -- -D warnings && exit 1

cargo clean
printf "\n\n*** All builds succeeded ***\n"
