cmake_minimum_required(VERSION 3.5)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/bin)
set(CMAKE_PREFIX_PATH ${CMAKE_CURRENT_SOURCE_DIR}/build/Release/generators)
set(CMAKE_OSX_DEPLOYMENT_TARGET "15.1")

if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release)
endif()

message(STATUS "Build type: ${CMAKE_BUILD_TYPE}")

project("C++ examples" LANGUAGES CXX C)

set(TARGET test)

if ("${CMAKE_BUILD_TYPE}" STREQUAL "Debug")
  set(CARGO_BUILD_PROFILE debug)
else()
  set(CARGO_BUILD_PROFILE release)
endif()

set(CXXBRIDGE_SOURCE_PATH ${CMAKE_CURRENT_SOURCE_DIR}/../target/cxxbridge)
set(CXXBRIDGE_LIB -L${CMAKE_CURRENT_SOURCE_DIR}/../target/${CARGO_BUILD_PROFILE} -lcxx_example)

# source files for examples
set(TEST_SRC
  ${CMAKE_CURRENT_SOURCE_DIR}/test.cpp
  ${CXXBRIDGE_SOURCE_PATH}/cxx_example/src/lib.rs.cc
)

add_executable(${TARGET} ${TEST_SRC})

if (${CMAKE_SYSTEM_NAME} MATCHES "Linux")
    set(DEPEND_LIBS "-lssl -lcrypto -ldl -lpthread")
elseif (${CMAKE_SYSTEM_NAME} MATCHES "Darwin")
    set(DEPEND_LIBS "-framework Security -framework CoreFoundation -framework SystemConfiguration")
endif()

# add libraries
target_link_libraries(${TARGET}
  PUBLIC
    ${CXXBRIDGE_LIB}
    ${DEPEND_LIBS}
)

# path to headerfile
target_include_directories(${TARGET}
  PRIVATE
    ${CXXBRIDGE_SOURCE_PATH}
)

install (TARGETS ${TARGET} DESTINATION .)
