# Copyright 2014 The GL-RS Developers. For a full listing of the authors,
# refer to the AUTHORS file at the top-level directory of this distribution.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

RUSTC               = rustc
RUSTDOC             = rustdoc

SRC_DIR             = src
TEST_DIR            = test
LIB_FILE            = $(SRC_DIR)/gl/lib.rs
GEN_FILE            = $(SRC_DIR)/gen/main.rs
EXAMPLE_FILES       = $(SRC_DIR)/examples/*.rs

# Generated previous OpenGL versions.
VERSIONS            = 21 30 31 32 33

define VERSION_LIB_FILES
VERSION_LIB_FILES_$(1)   := $(SRC_DIR)/gl/gl$(1).rs
endef

$(foreach vers,$(VERSIONS),$(eval $(call VERSION_LIB_FILES,$(vers))))

DOC_DIR             = doc
EXAMPLES_DIR        = examples
LIB_DIR             = lib

all: gen lib examples doc

lib:
	@mkdir -p $(LIB_DIR)
	$(RUSTC) --out-dir=$(LIB_DIR) -O $(LIB_FILE)

# Targets for previous OpenGL versions
define TARGET_CRATES
lib-$(1): $(VERSION_LIB_FILES_$(1))
	@mkdir -p $(LIB_DIR)
	$(RUSTC) --out-dir=$(LIB_DIR) -O $(VERSION_LIB_FILES_$(1))
endef

$(foreach vers,$(VERSIONS),$(eval $(call TARGET_CRATES,$(vers))))	
	
check:
	@mkdir -p $(TEST_DIR)
	$(RUSTC) --out-dir=$(TEST_DIR) --test $(LIB_FILE)
	$(TEST_DIR)/gl

doc: 
	@mkdir -p $(DOC_DIR)
	$(RUSTDOC) -o $(DOC_DIR) $(LIB_FILE)

examples-dir:
	mkdir -p $(EXAMPLES_DIR)

examples-deps:
	make lib -C deps/glfw-rs

$(EXAMPLE_FILES): lib examples-dir examples-deps
	$(RUSTC) -L deps/glfw-rs/lib -L $(LIB_DIR) --out-dir=$(EXAMPLES_DIR) $@

examples: $(EXAMPLE_FILES)

gen-deps:
	make lib -C deps/sax-rs

gen: gen-deps
	@mkdir -p bin
	$(RUSTC) -L deps/sax-rs/lib $(GEN_FILE) -o bin/glrsgen

clean:
	rm -rf $(LIB_DIR)
	rm -rf $(TEST_DIR)
	rm -rf $(DOC_DIR)
	make clean -C deps/glfw-rs
	make clean -C deps/sax-rs

.PHONY: \
	all \
	lib \
	check \
	doc \
	examples \
	examples-deps\
	examples-dir \
	gen \
	gen-deps \
	$(EXAMPLE_FILES) \
	clean
