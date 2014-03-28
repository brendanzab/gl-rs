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

# Generated previous OpenGL versions.
VERSIONS            = 21 30 31 33

define VERSION_LIB_FILES
VERSION_LIB_FILES_$(1)   := $(SRC_DIR)/gl/gl$(1).rs
endef

$(foreach vers,$(VERSIONS),$(eval $(call VERSION_LIB_FILES,$(vers))))

CRATE_NAME          = $(shell $(RUSTC) --crate-name $(LIB_FILE))
CRATE_FILES         = $(shell $(RUSTC) --crate-file-name $(LIB_FILE))

EXAMPLES            = basic triangle
EXAMPLES_DIR        = examples

DOC_DIR             = doc
LIB_DIR             = lib

INSTALL_PREFIX      = /usr/local
LIB_INSTALL_DIR     = $(INSTALL_PREFIX)/lib

all: lib doc

lib: $(CRATE_FILES)

$(CRATE_FILES): $(LIB_FILE)
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

examples: $(EXAMPLES:%=$(EXAMPLES_DIR)/%)

$(EXAMPLES_DIR)/% : $(EXAMPLES:%=src/examples/%/main.rs)
	@mkdir -p $(EXAMPLES_DIR)
	@echo $(RUSTC) -L/usr/lib -L/usr/local/lib -Llib --out-dir=$(EXAMPLES_DIR) -O src/examples/$*/main.rs

gen: src/gen/main.rs
	@mkdir -p bin
	$(RUSTC) -L/usr/lib -L/usr/local/lib -Llib -O $? -o bin/glrsgen

install: lib
	@mkdir -p $(LIB_INSTALL_DIR)
	@ $(foreach crate, $(CRATE_FILES), \
		cp $(LIB_DIR)/$(crate) $(LIB_INSTALL_DIR)/$(crate) && \
		echo "Installed $(crate) to $(LIB_INSTALL_DIR)" ; \
	)

uninstall:
	@-rm $(LIB_INSTALL_DIR)/lib$(CRATE_NAME)-*.rlib ||:

clean:
	rm -rf $(LIB_DIR)
	rm -rf $(TEST_DIR)
	rm -rf $(DOC_DIR)

.PHONY: \
	all \
	lib \
	check \
	doc \
	install \
	uninstall \
	clean
