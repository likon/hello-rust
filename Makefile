# 
# Copyright (C) 2006-2013 OpenWrt.org
#
# This is free software, licensed under the GNU General Public License v2.
# See /LICENSE for more information.
#

include $(TOPDIR)/rules.mk

PKG_NAME:=hello-rust
PKG_RELEASE:=1

include $(INCLUDE_DIR)/package.mk

define Package/hello-rust
  SECTION:=hello-rust
  CATEGORY:=Rust Program
  TITLE:=just a test for rust lang
  MAINTAINER:=likon
  DEPENDS:= +libubox +libubus +libuci
endef

define Build/Prepare
	mkdir -p $(PKG_BUILD_DIR)/.cargo
	$(CP) ./src/* $(PKG_BUILD_DIR)/
	$(CP) ./src/.cargo/config $(PKG_BUILD_DIR)/.cargo
endef

define Build/Compile
	$(MAKE) -C $(PKG_BUILD_DIR) \
		CC="$(TARGET_CC)" \
		CFLAGS="$(TARGET_CFLAGS) -Wall"
endef

define Package/hello-rust/install
	$(INSTALL_DIR) $(1)/usr/bin $(1)/etc/config $(1)/etc/init.d
	$(STRIP) $(PKG_BUILD_DIR)/target/mipsel-unknown-linux-gnu/release/hello-rust
	$(INSTALL_BIN) $(PKG_BUILD_DIR)/target/mipsel-unknown-linux-gnu/release/hello-rust $(1)/usr/bin/
endef

$(eval $(call BuildPackage,hello-rust))

