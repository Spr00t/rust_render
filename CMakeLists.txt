project(helper)
cmake_minimum_required(VERSION 2.8)


find_package(Qt5Core)
find_package(Qt5Test)
find_package(Qt5Gui)
find_package(Qt5Widgets)
SET(CMAKE_PREFIX_PATH "~/Qt/5.4/clang_64/")

cmake_minimum_required(VERSION 2.8)
SET (CMAKE_BUILD_TYPE Debug)

set(DATA_FILE
    ${PROJECT_SOURCE_DIR}/src/main.rs
    ${PROJECT_SOURCE_DIR}/src/renderer.rs
)

#add_custom_target(FUZZY SOURCES ${DATA_FILE})
include_directories(${Qt5Core_INCLUDE_DIRS} ${Qt5Test_INCLUDE_DIRS} ${Qt5Gui_INCLUDE_DIRS} ${Qt5Widgets_INCLUDE_DIRS})

# Use the compile definitions defined in the Qt 5 Widgets module
add_definitions(${Qt5Core_DEFINITIONS} ${Qt5Test_DEFINITIONS}
    #${Qt5Gui_DEFINITIONS} ${Qt5Widgets_DEFINITIONS}
    )

MESSAGE(${Qt5Gui_DEFINITIONS} ${Qt5Widgets_DEFINITIONS})

set(SRC_LIST_COMMON
    ${CMAKE_CURRENT_SOURCE_DIR}/window.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/helper.cpp

)

set(SRC_LIST_LIB
   ${SRC_LIST_COMMON}
)

set(SRC_LIST_APP
   ${SRC_LIST_COMMON}
   ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp
)

SET(CMAKE_CXX_FLAGS ${CMAKE_CXX_FLAGS} "-std=c++11 -stdlib=libc++")
SET(CMAKE_EXE_LINKER_FLAGS ${CMAKE_EXE_LINKER_FLAGS} "-stdlib=libc++")




set(UI_SOURCES
    window.ui
)

#
# Generate necessary headers from .ui files. (qmake lets `uic` do this job.)
# hint from [Cross-platform Qt5 project using cmake](http://stackoverflow.com/questions/21174586/cross-platform-qt5-project-using-cmake)
#


qt5_wrap_ui(UI_GENERATED_HEADERS ${UI_SOURCES})
#qt5_add_resources(RES ${CMAKE_CURRENT_SOURCE_DIR}/res.qrc)

# Find includes in corresponding build directories
set(CMAKE_INCLUDE_CURRENT_DIR ON)

# Instruct CMake to run moc automatically when needed.
set(CMAKE_AUTOMOC ON)

#add_executable(${PROJECT_NAME}_test ${SRC_LIST_TEST})
#target_link_libraries(${PROJECT_NAME}_test Qt5::Core Qt5::Test)

add_executable(test_app main.cpp )

add_library(helper SHARED ${SRC_LIST_LIB} ${UI_GENERATED_HEADERS} ${RES})


target_link_libraries(test_app helper)
target_link_libraries(helper
    LINK_PRIVATE Qt5::Core
    LINK_PRIVATE Qt5::Test
    LINK_PRIVATE Qt5::Gui
    LINK_PRIVATE Qt5::Widgets)

#install(TARGETS helper ARCHIVE DESTINATION lib)




