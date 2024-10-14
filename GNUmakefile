INSTALL = install
INSTALL_PROGRAM = $(INSTALL)
INSTALL_DATA = $(INSTALL) -m 644

bindir = /bin
datadir = /share
prefix = ./dist
DESTDIR = $(prefix)

FILES =
FILES += background/background.mjs
FILES += common/common.mjs
FILES += common/message-collector.mjs
FILES += content/content.js
FILES += icons/noematic-48.png
FILES += popup/popup.css
FILES += popup/popup.html
FILES += popup/popup.mjs
FILES += search/index.css
FILES += search/index.html
FILES += search/search-result.mjs
FILES += search/search.css
FILES += search/search.html
FILES += search/search.mjs
FILES += search/shared.css

SRCS = $(addprefix extension/,$(FILES))

MOZILLA_DIR = $(DESTDIR)$(datadir)/mozilla/extensions/noematic
MOZILLA_TARGETS = $(addprefix $(MOZILLA_DIR)/,$(FILES))
MOZILLA_TARGETS += $(MOZILLA_DIR)/manifest.json

CHROMIUM_DIR = $(DESTDIR)$(datadir)/chromium/extensions/noematic
CHROMIUM_TARGETS = $(addprefix $(CHROMIUM_DIR)/,$(FILES))
CHROMIUM_TARGETS += $(CHROMIUM_DIR)/manifest.json

.PHONY: all
all: $(MOZILLA_TARGETS) $(CHROMIUM_TARGETS)

$(MOZILLA_DIR)/manifest.json:: extension/manifest.firefox.json
	$(INSTALL_DATA) -D $< $@

$(CHROMIUM_DIR)/manifest.json:: extension/manifest.chromium.json
	$(INSTALL_DATA) -D $< $@

$(MOZILLA_DIR)/%:: extension/%
	$(INSTALL_DATA) -D $< $@

$(CHROMIUM_DIR)/%:: extension/%
	$(INSTALL_DATA) -D $< $@

.PHONY: clean
clean:
	rm -rf $(DESTDIR)
