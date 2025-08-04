#++++++++++++++++
#.IDENTIFICATION Makefile
#.LANGUAGE       Make (makefile script)
#.AUTHOR         Marjorie Lapointe
#.ENVIRONMENT    Debian
#.KEYWORDS		 
#.VERSION  1.0   29-Jul-2025
#.PURPOSE        Install the extension healpix_pgrx
#----------------

PACK=install
SHELL=/bin/sh

PG_VERSION = 17
EXT_NAME = healpix_pgrx
VERSION=0.0.1

# PostgreSQL repositories
PG_LIB_DIR = /usr/lib/postgresql/$(PG_VERSION)/lib
PG_SHARE_DIR = /usr/share/postgresql/$(PG_VERSION)/extension

# Source files directory
SOURCE_DIR = cds_healpix_pgrx

# Targets
.PHONY: install clean show

install:
	sudo apt install postgresql-$(PG_VERSION)
	sudo cp $(SOURCE_DIR)/lib/postgresql/$(PG_VERSION)/lib/$(EXT_NAME).so $(PG_LIB_DIR)/
	sudo cp $(SOURCE_DIR)/share/postgresql/$(PG_VERSION)/extension/$(EXT_NAME).control $(PG_SHARE_DIR)/
	sudo cp $(SOURCE_DIR)/share/postgresql/$(PG_VERSION)/extension/$(EXT_NAME)--$(VERSION).sql $(PG_SHARE_DIR)/
	sudo chmod 777 $(PG_LIB_DIR)/$(EXT_NAME).so
	sudo chmod 777 $(PG_SHARE_DIR)/$(EXT_NAME).control
	sudo chmod 777 $(PG_SHARE_DIR)/$(EXT_NAME)--$(VERSION).sql
	echo "Extension $(EXT_NAME) installée pour PostgreSQL $(PG_VERSION)" 

clean:
	sudo rm -f $(PG_LIB_DIR)/$(EXT_NAME).so
	sudo rm -f $(PG_SHARE_DIR)/$(EXT_NAME).control
	sudo rm -f $(PG_SHARE_DIR)/$(EXT_NAME)--$(VERSION).sql
	echo "Extension $(EXT_NAME) supprimée"

show:
	echo "Lib directory:     $(PG_LIB_DIR)"
	echo "Share directory:   $(PG_SHARE_DIR)"
	echo "Source directory:  $(SOURCE_DIR)"