cmake_minimum_required(VERSION 3.20)
project(NativeBanglaIME LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_definitions(-DUNICODE -D_UNICODE -DWIN32_LEAN_AND_MEAN)

include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}/include
    ${CMAKE_CURRENT_SOURCE_DIR}/src
)

set(SOURCES
    src/DllMain.cpp
    src/ClassFactory.cpp
    src/TextService.cpp
    src/Composition.cpp
    src/CandidateList.cpp
    src/NativeBanglaIME.def
    resources/NativeBanglaIME.rc
)

set(HEADERS
    include/NativeBanglaIME.h
    src/Bridge.h
    src/CandidateList.h
    src/ClassFactory.h
    src/Composition.h
    src/TextService.h
)

add_library(NativeBanglaIME SHARED ${SOURCES} ${HEADERS})

target_link_libraries(NativeBanglaIME PRIVATE
    msctf
    ole32
    advapi32
    user32
    gdi32
)

set_target_properties(NativeBanglaIME PROPERTIES
    OUTPUT_NAME "NativeBanglaIME"
    PREFIX ""
    SUFFIX ".dll"
)
