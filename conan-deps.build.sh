#!/bin/bash
set -o errexit
set -o pipefail

[[ -z ${ARCH} ]]                      && echo "error: ARCH not set"                       && exit 1;
[[ -z ${BUILD_DIR} ]]                 && echo "error: BUILD_DIR not set"                  && exit 1;
[[ -z ${CONAN_REMOTE} ]]              && echo "error: CONAN_REMOTE not set"               && exit 1;
[[ -z ${CONAN_USER_HOME} ]]           && echo "error: CONAN_USER_HOME not set"            && exit 1;
[[ -z ${SERVICE} ]]                   && echo "error: SERVICE not set"                    && exit 1;
[[ -z ${SOURCE_DIR} ]]                && echo "error: SOURCE_DIR not set"                 && exit 1;

echo ARCH=${ARCH}
if [ ! "${BUILD_DIR:0:1}" == "/" ]; then BUILD_DIR=$(pwd)/${BUILD_DIR}; fi
echo BUILD_DIR=${BUILD_DIR}
export CONAN_CPU_COUNT=$(getconf _NPROCESSORS_ONLN)
echo CONAN_CPU_COUNT=${CONAN_CPU_COUNT}
echo CONAN_REMOTE=${CONAN_REMOTE}
echo CONAN_SSL_VERIFICATION=${CONAN_SSL_VERIFICATION}
CONAN_USER_HOME=${BUILD_DIR}
echo CONAN_USER_HOME=${CONAN_USER_HOME}

conan remote add my-conan-remote ${CONAN_REMOTE} ${CONAN_SSL_VERIFICATION} -f --insert

[[ "${ARCH}" != "x86_64" ]] && NO_TEST="-tf=None"

mkdir -p ${BUILD_DIR}/${SERVICE}
cd ${BUILD_DIR}/${SERVICE}

# For 'enrollment' we need 'libprovisioning_service_client' and thus
# enable 'use_prov_client'. Therefore we have two conan config files for
# 'azure-iot-sdk-c'.
if [ "${SERVICE}" == "enrollment" ] || [ "${SERVICE}" == "enrollment-dev" ];
then
  cp  ${SOURCE_DIR}/simulator/conanfile-prov.txt conanfile.txt;
else
  cp  ${SOURCE_DIR}/simulator/conanfile.txt conanfile.txt;
fi

conan install . --profile:host=/conan.${ARCH}.profile --profile:build=/conan.x86_64.profile --build missing ${NO_TEST}
conan upload "*" -r my-conan-remote --all -c
