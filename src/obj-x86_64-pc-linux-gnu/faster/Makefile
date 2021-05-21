default:
TOPSRCDIR = /worker/build
TOPOBJDIR = /worker/build/obj-x86_64-pc-linux-gnu
MDDEPDIR = .deps
TOUCH ?= touch
include $(TOPSRCDIR)/config/makefiles/functions.mk
include $(TOPSRCDIR)/config/AB_rCD.mk
AB_CD = en-US
PYTHON3 = /worker/build/obj-x86_64-pc-linux-gnu/_virtualenvs/init_py3/bin/python
ACDEFINES = -DACCESSIBILITY=1 -DATK_MAJOR_VERSION=2 -DATK_MINOR_VERSION=36 -DATK_REV_VERSION=0 -DBROWSER_CHROME_URL=chrome://browser/content/browser.xhtml -DBROWSER_CHROME_URL_QUOTED='"chrome://browser/content/browser.xhtml"' -DBUILD_CTYPES=1 -DCROSS_COMPILE='' -DENABLE_REMOTE_AGENT=1 -DENABLE_SHARED_MEMORY=1 -DENABLE_SYSTEM_EXTENSION_DIRS=1 -DENABLE_TESTS=1 -DENABLE_WASM_MULTI_VALUE=1 -DENABLE_WASM_REFTYPES=1 -DENABLE_WASM_SIMD=1 -DENABLE_WASM_SIMD_WORMHOLE=1 -DFORCE_PR_LOG=1 -DFUNCPROTO=15 -DGDK_VERSION_MAX_ALLOWED=GDK_VERSION_3_14 -DGLIB_VERSION_MAX_ALLOWED=GLIB_VERSION_2_42 -DGLIB_VERSION_MIN_REQUIRED=GLIB_VERSION_2_42 -DGL_PROVIDER_EGL=1 -DGTEST_HAS_RTTI=0 -DHAVE_64BIT_BUILD=1 -DHAVE_ALLOCA_H=1 -DHAVE_BYTESWAP_H=1 -DHAVE_CLOCK_MONOTONIC=1 -DHAVE_CPUID_H=1 -DHAVE_DIRENT_H=1 -DHAVE_DLADDR=1 -DHAVE_DLOPEN=1 -DHAVE_FONTCONFIG_FCFREETYPE_H=1 -DHAVE_FTS_H=1 -DHAVE_FT_BITMAP_SIZE_Y_PPEM=1 -DHAVE_FT_GLYPHSLOT_EMBOLDEN=1 -DHAVE_FT_LOAD_SFNT_TABLE=1 -DHAVE_GETCONTEXT=1 -DHAVE_GETOPT_H=1 -DHAVE_GETPAGESIZE=1 -DHAVE_GETTID=1 -DHAVE_GMTIME_R=1 -DHAVE_INTTYPES_H=1 -DHAVE_LCHOWN=1 -DHAVE_LINUX_IF_ADDR_H=1 -DHAVE_LINUX_PERF_EVENT_H=1 -DHAVE_LINUX_QUOTA_H=1 -DHAVE_LINUX_RTNETLINK_H=1 -DHAVE_LOCALECONV=1 -DHAVE_LOCALTIME_R=1 -DHAVE_LSTAT64=1 -DHAVE_LUTIMES=1 -DHAVE_MALLINFO=1 -DHAVE_MALLOC_H=1 -DHAVE_MALLOC_USABLE_SIZE=1 -DHAVE_MEMALIGN=1 -DHAVE_MEMFD_CREATE=1 -DHAVE_MEMMEM=1 -DHAVE_NETINET_IN_H=1 -DHAVE_NL_TYPES_H=1 -DHAVE_POSIX_FADVISE=1 -DHAVE_POSIX_FALLOCATE=1 -DHAVE_POSIX_MEMALIGN=1 -DHAVE_PTHREAD_H=1 -DHAVE_RES_NINIT=1 -DHAVE_SETPRIORITY=1 -DHAVE_STAT64=1 -DHAVE_STATFS=1 -DHAVE_STATFS64=1 -DHAVE_STATVFS=1 -DHAVE_STATVFS64=1 -DHAVE_STDINT_H=1 -DHAVE_STRERROR=1 -DHAVE_STRNDUP=1 -DHAVE_SYSCALL=1 -DHAVE_SYS_MOUNT_H=1 -DHAVE_SYS_QUEUE_H=1 -DHAVE_SYS_QUOTA_H=1 -DHAVE_SYS_STATFS_H=1 -DHAVE_SYS_STATVFS_H=1 -DHAVE_SYS_TYPES_H=1 -DHAVE_SYS_VFS_H=1 -DHAVE_THREAD_TLS_KEYWORD=1 -DHAVE_TRUNCATE64=1 -DHAVE_UNISTD_H=1 -DHAVE_VALLOC=1 -DHAVE_VA_COPY=1 -DHAVE_VA_LIST_AS_ARRAY=1 -DHAVE_VISIBILITY_ATTRIBUTE=1 -DHAVE_VISIBILITY_HIDDEN_ATTRIBUTE=1 -DHAVE__UNWIND_BACKTRACE=1 -DHAVE___CXA_DEMANGLE=1 -DJS_64BIT=1 -DJS_CODEGEN_X64=1 -DJS_DEFAULT_JITREPORT_GRANULARITY=3 -DJS_HAS_CTYPES=1 -DJS_HAS_INTL_API=1 -DJS_PUNBOX64=1 -DMALLOC_H='<malloc.h>' -DMALLOC_USABLE_SIZE_CONST_PTR='' -DMOZILLA_UAVERSION='"87.0"' -DMOZILLA_VERSION='"87.0"' -DMOZILLA_VERSION_U=87.0 -DMOZ_AARCH64_JSCVT=0 -DMOZ_ACCESSIBILITY_ATK=1 -DMOZ_APP_UA_NAME='""' -DMOZ_APP_UA_VERSION='"87.0"' -DMOZ_AV1=1 -DMOZ_BLOCK_PROFILE_DOWNGRADE=1 -DMOZ_BUILD_APP=browser -DMOZ_BUNDLED_FONTS=1 -DMOZ_CRASHREPORTER=1 -DMOZ_DATA_REPORTING=1 -DMOZ_DAV1D_ASM=1 -DMOZ_DEDICATED_PROFILES=1 -DMOZ_DISTRIBUTION_ID='"co.konkehq.browser"' -DMOZ_DLL_PREFIX='"lib"' -DMOZ_DLL_SUFFIX='".so"' -DMOZ_ENABLE_DBUS=1 -DMOZ_ENABLE_FORKSERVER=1 -DMOZ_ENABLE_SKIA=1 -DMOZ_FFMPEG=1 -DMOZ_FFVPX=1 -DMOZ_FMP4=1 -DMOZ_GECKO_PROFILER=1 -DMOZ_GECKO_PROFILER_PARSE_ELF=1 -DMOZ_GLEAN=1 -DMOZ_GLUE_IN_PROGRAM=1 -DMOZ_HAS_REMOTE=1 -DMOZ_INSTRUMENT_EVENT_LOOP=1 -DMOZ_LIBAV_FFT=1 -DMOZ_LOGGING=1 -DMOZ_MACBUNDLE_ID=co.konkehq.browser.co.konkehq.browser -DMOZ_MEMORY=1 -DMOZ_NORMANDY=1 -DMOZ_OXIDIZED_BREAKPAD=1 -DMOZ_PEERCONNECTION=1 -DMOZ_PLACES=1 -DMOZ_PROFILER_MEMORY=1 -DMOZ_PULSEAUDIO=1 -DMOZ_RAW=1 -DMOZ_RUST_SIMD=1 -DMOZ_SAMPLE_TYPE_FLOAT32=1 -DMOZ_SANDBOX=1 -DMOZ_SCTP=1 -DMOZ_SERVICES_HEALTHREPORT=1 -DMOZ_SRTP=1 -DMOZ_STATIC_JS=1 -DMOZ_TREE_CAIRO=1 -DMOZ_TREE_PIXMAN=1 -DMOZ_UPDATER=1 -DMOZ_UPDATE_CHANNEL=release -DMOZ_USER_DIR='".mozilla"' -DMOZ_VERIFY_MAR_SIGNATURE=1 -DMOZ_VORBIS=1 -DMOZ_WAYLAND=1 -DMOZ_WEBM_ENCODER=1 -DMOZ_WEBRTC=1 -DMOZ_WEBRTC_ASSERT_ALWAYS=1 -DMOZ_WEBRTC_SIGNALING=1 -DMOZ_WEBSPEECH=1 -DMOZ_WEBSPEECH_TEST_BACKEND=1 -DMOZ_WIDGET_GTK=1 -DMOZ_X11=1 -DMOZ_XUL=1 -DNECKO_WIFI=1 -DNO_NSPR_10_SUPPORT=1 -DNS_PRINTING=1 -DNS_PRINT_PREVIEW=1 -DRELEASE_OR_BETA=1 -DSTATIC_JS_API=1 -DSTDC_HEADERS=1 -DTARGET_XPCOM_ABI='"x86_64-gcc3"' -DUSE_SKIA=1 -DU_STATIC_IMPLEMENTATION=1 -DU_USING_ICU_NAMESPACE=0 -DVA_COPY=va_copy -DXP_LINUX=1 -DXP_UNIX=1 -D_REENTRANT=1
MOZ_BUILD_APP = browser
MOZ_WIDGET_TOOLKIT = gtk
INSTALL_MANIFESTS = dist/bin dist/bin/browser dist/bin/browser/features/formautofill@mozilla.org dist/bin/browser/features/webcompat-reporter@mozilla.org dist/bin/browser/localization dist/bin/gmp-clearkey/0.1 dist/bin/gmp-fake/1.0 dist/bin/gmp-fakeopenh264/1.0 dist/bin/localization
install-dist/bin: $(TOPOBJDIR)/build/application.ini $(TOPOBJDIR)/toolkit/components/featuregates/feature_definitions.json $(TOPOBJDIR)/toolkit/components/telemetry/EventArtifactDefinitions.json $(TOPOBJDIR)/toolkit/components/telemetry/ScalarArtifactDefinitions.json $(TOPOBJDIR)/toolkit/library/build/dependentlibs.list $(TOPOBJDIR)/toolkit/locales/locale.ini $(TOPOBJDIR)/toolkit/locales/multilocale.txt $(TOPOBJDIR)/toolkit/locales/update.locale $(TOPOBJDIR)/toolkit/mozapps/extensions/built_in_addons.json
install-dist/bin/browser: $(TOPOBJDIR)/browser/base/content/aboutNetErrorCodes.js $(TOPOBJDIR)/browser/locales/bookmarks.html $(TOPOBJDIR)/browser/locales/updater.ini $(TOPOBJDIR)/devtools/shared/webconsole/reserved-js-words.js
include $(TOPSRCDIR)/config/faster/rules.mk
default: $(TOPOBJDIR)/$(MDDEPDIR)/mozilla-config.h.stub
$(TOPOBJDIR)/mozilla-config.h: $(TOPOBJDIR)/$(MDDEPDIR)/mozilla-config.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/$(MDDEPDIR)/mozilla-config.h.pp
$(TOPOBJDIR)/$(MDDEPDIR)/mozilla-config.h.stub: /worker/build/python/mozbuild/mozbuild/action/process_define_files.py $(TOPSRCDIR)/mozilla-config.h.in
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/process_define_files.py process_define_file $(TOPOBJDIR)/mozilla-config.h $(TOPOBJDIR)/$(MDDEPDIR)/mozilla-config.h.pp $(TOPOBJDIR)/$(MDDEPDIR)/mozilla-config.h.stub $(TOPSRCDIR)/mozilla-config.h.in)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/$(MDDEPDIR)/buildid.h.stub
$(TOPOBJDIR)/buildid.h: $(TOPOBJDIR)/$(MDDEPDIR)/buildid.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/$(MDDEPDIR)/buildid.h.pp
$(TOPOBJDIR)/$(MDDEPDIR)/buildid.h.stub: /worker/build/build/variables.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/variables.py buildid_header $(TOPOBJDIR)/buildid.h $(TOPOBJDIR)/$(MDDEPDIR)/buildid.h.pp $(TOPOBJDIR)/$(MDDEPDIR)/buildid.h.stub)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/$(MDDEPDIR)/source-repo.h.stub
$(TOPOBJDIR)/source-repo.h: $(TOPOBJDIR)/$(MDDEPDIR)/source-repo.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/$(MDDEPDIR)/source-repo.h.pp
$(TOPOBJDIR)/$(MDDEPDIR)/source-repo.h.stub: /worker/build/build/variables.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/variables.py source_repo_header $(TOPOBJDIR)/source-repo.h $(TOPOBJDIR)/$(MDDEPDIR)/source-repo.h.pp $(TOPOBJDIR)/$(MDDEPDIR)/source-repo.h.stub)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/build/$(MDDEPDIR)/application.ini.stub
$(TOPOBJDIR)/build/application.ini: $(TOPOBJDIR)/build/$(MDDEPDIR)/application.ini.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/build/$(MDDEPDIR)/application.ini.pp
$(TOPOBJDIR)/build/$(MDDEPDIR)/application.ini.stub: /worker/build/python/mozbuild/mozbuild/action/preprocessor.py $(TOPSRCDIR)/build/application.ini.in
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/preprocessor.py generate $(TOPOBJDIR)/build/application.ini $(TOPOBJDIR)/build/$(MDDEPDIR)/application.ini.pp $(TOPOBJDIR)/build/$(MDDEPDIR)/application.ini.stub $(TOPSRCDIR)/build/application.ini.in -DGRE_MILESTONE=87.0 -DMAR_CHANNEL_ID= -DMOZ_APPUPDATE_HOST=push.dothq.co -DMOZ_APP_BASENAME=Konke '-DMOZ_APP_DISPLAYNAME=Konke Browser' '-DMOZ_APP_ID={ec8030f7-c20a-464f-9b0e-13a3a9e97384}' -DMOZ_APP_PROFILE=konke -DMOZ_APP_REMOTINGNAME=konke '-DMOZ_APP_VENDOR=Konke HQ' -DMOZ_APP_VERSION=87.0 -DMOZ_BUILD_APP_IS_BROWSER=1 -DMOZ_CRASHREPORTER=1 -DMOZ_PROFILE_MIGRATOR=1 -DMOZ_UPDATER=1 -DTOPOBJDIR=/worker/build/obj-x86_64-pc-linux-gnu)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/build/$(MDDEPDIR)/application.ini.h.stub
$(TOPOBJDIR)/build/application.ini.h: $(TOPOBJDIR)/build/$(MDDEPDIR)/application.ini.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/build/$(MDDEPDIR)/application.ini.h.pp
$(TOPOBJDIR)/build/$(MDDEPDIR)/application.ini.h.stub: /worker/build/build/appini_header.py $(TOPOBJDIR)/build/application.ini
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/appini_header.py main $(TOPOBJDIR)/build/application.ini.h $(TOPOBJDIR)/build/$(MDDEPDIR)/application.ini.h.pp $(TOPOBJDIR)/build/$(MDDEPDIR)/application.ini.h.stub $(TOPOBJDIR)/build/application.ini)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/build/unix/elfhack/inject/$(MDDEPDIR)/x86_64.c.stub
$(TOPOBJDIR)/build/unix/elfhack/inject/x86_64.c: $(TOPOBJDIR)/build/unix/elfhack/inject/$(MDDEPDIR)/x86_64.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/build/unix/elfhack/inject/$(MDDEPDIR)/x86_64.c.pp
$(TOPOBJDIR)/build/unix/elfhack/inject/$(MDDEPDIR)/x86_64.c.stub: /worker/build/build/unix/elfhack/inject/copy_source.py $(TOPSRCDIR)/build/unix/elfhack/inject.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/unix/elfhack/inject/copy_source.py copy $(TOPOBJDIR)/build/unix/elfhack/inject/x86_64.c $(TOPOBJDIR)/build/unix/elfhack/inject/$(MDDEPDIR)/x86_64.c.pp $(TOPOBJDIR)/build/unix/elfhack/inject/$(MDDEPDIR)/x86_64.c.stub $(TOPSRCDIR)/build/unix/elfhack/inject.c)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub
$(TOPOBJDIR)/config/stl.sentinel: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/new: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/algorithm: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/atomic: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/cassert: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/climits: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/cmath: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/condition_variable: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/cstdarg: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/cstdio: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/cstdlib: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/cstring: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/cwchar: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/deque: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/functional: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/ios: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/iosfwd: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/iostream: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/istream: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/iterator: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/limits: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/list: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/map: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/memory: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/mutex: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/ostream: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/regex: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/set: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/shared_mutex: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/stack: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/string: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/thread: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/tuple: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/type_traits: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/unordered_map: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/unordered_set: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/utility: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/vector: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
$(TOPOBJDIR)/dist/stl_wrappers/xutility: $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.pp
$(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub: /worker/build/config/make-stl-wrappers.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/config/make-stl-wrappers.py gen_wrappers $(TOPOBJDIR)/config/stl.sentinel $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.pp $(TOPOBJDIR)/config/$(MDDEPDIR)/stl.sentinel.stub /worker/build/obj-x86_64-pc-linux-gnu/dist/stl_wrappers gcc /worker/build/config/gcc-stl-wrapper.template.h new algorithm atomic cassert climits cmath condition_variable cstdarg cstdio cstdlib cstring cwchar deque functional ios iosfwd iostream istream iterator limits list map memory mutex ostream regex set shared_mutex stack string thread tuple type_traits unordered_map unordered_set utility vector xutility)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub
$(TOPOBJDIR)/config/system-header.sentinel: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/new: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/algorithm: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/atomic: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cassert: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/climits: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cmath: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/condition_variable: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cstdarg: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cstdio: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cstdlib: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cstring: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cwchar: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/deque: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/functional: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ios: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/iosfwd: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/iostream: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/istream: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/iterator: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/limits: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/list: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/map: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/memory: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/mutex: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ostream: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/regex: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/set: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/shared_mutex: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/stack: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/string: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/thread: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/tuple: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/type_traits: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/unordered_map: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/unordered_set: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/utility: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/vector: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/xutility: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/A4Stuff.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/activscp.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/AEDataModel.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/AEObjects.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/AEPackObject.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/AERegistry.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/AEUtils.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/afxcmn.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/afxcoll.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/afxcview.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/afxdisp.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/afxdtctl.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/afxext.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/afxmt.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/afxpriv.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/afxtempl.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/afxwin.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Aliases.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/all.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/alloca.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/alloc.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/alsa/asoundlib.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ansi_parms.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/a.out.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/app/Cursor.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Appearance.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/AppFileInfo.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/AppKit.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/AppleEvents.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Application.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/app/Message.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/app/MessageRunner.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/arpa/inet.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/arpa/nameser.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/array: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/asm/page.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/asm/sigcontext.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/asm/signal.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ASRegistry.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/assert.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/atk/atk.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/atlcom.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/atlconv.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/atlctl.cpp: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/atlctl.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ATLCTL.H: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/atlhost.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/atlimpl.cpp: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/atlwin.cpp: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/atomic.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ATSTypes.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ATSUnicode.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Balloons.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/base64.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/base/pblock.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/base/PCR_Base.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/base/session.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/basetyps.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/be/app/Application.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Beep.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/be/kernel/image.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/be/kernel/OS.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/bfd.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Bitmap.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/bitset: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/blapi.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/blapit.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/bsd/libc.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/bsd/syscall.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/bstring.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/builtin.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Button.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/byteswap.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cairo-atsui.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cairo-beos.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cairo-directfb.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cairo-ft.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cairo-glitz.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cairo-gobject.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cairo.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cairo-pdf.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cairo-ps.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cairo-qpainter.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cairo-qt.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cairo-quartz.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cairo-tee.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cairo-win32.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cairo-xlib.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cairo-xlib-xrender.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/callconv.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Carbon/Carbon.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/CarbonEvents.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Carbon.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/c_asm.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cctype: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cderr.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cerrno: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/certdb.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cert.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/certt.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/CFBase.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/CFBundle.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/CFData.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/CFDictionary.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cf.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/CFNumber.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/CFPlugIn.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/CFPreferences.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/CFString.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/CFURL.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/CGAffineTransform.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/CheckBox.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ciferfam.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Clipboard.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cmmf.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cmmft.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cmplrs/stsupport.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cms.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cmsreclist.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cmst.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Cocoa/Cocoa.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/CodeFragments.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/comdef.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/commctrl.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/COMMCTRL.H: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/commdlg.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/compat.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/complex: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/condapi.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ConditionalMacros.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/conio.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/console.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ControlDefinitions.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Controls.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/CoreFoundation/CoreFoundation.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/CoreServices/CoreServices.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/CPalmRec.cpp: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Cpalmrec.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/CPCatgry.cpp: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/CPDbBMgr.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/CPString.cpp: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/CPString.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/crmf.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/crmft.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/crtdbg.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/crt_externs.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/crypt.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cryptohi.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cryptoht.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cstddef: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ctime: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ctype.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/curl/curl.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/curl/easy.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/curses.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/cxxabi.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/DateTimeUtils.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/dbus/dbus-glib.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/dbus/dbus-glib-lowlevel.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/dbus/dbus.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ddeml.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Debug.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/dem.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/descrip.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Devices.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/dfb_types.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/dfiff.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/dgiff.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Dialogs.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/direct/build.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/direct/clock.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/direct/conf.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/direct/debug.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/direct/direct.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/directfb_keyboard.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/directfb_strings.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/direct.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/direct/hash.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/direct/interface.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/direct/interface_implementation.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/direct/list.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/direct/log.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/direct/memcpy.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/direct/mem.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/direct/messages.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/direct/modules.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/direct/serial.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/direct/signals.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/direct/stream.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/direct/system.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/direct/thread.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/direct/trace.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/direct/tree.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/direct/types.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/direct/utf8.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/direct/util.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/dirent.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/DiskInit.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/dlfcn.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/dlgs.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/dl.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/docobj.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/dos/dosextens.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/dos.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Drag.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/DriverServices.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/DriverSynchronization.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/DropInPanel.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/dvidef.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ecl-exp.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/elf.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/endian.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Entry.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/err.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/errno.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Errors.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Events.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/exception: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/exdisp.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ExDisp.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/exe386.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/execinfo.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/extras.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fcntl.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/features.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fenv.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ffi.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fibdef.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/File.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/filehdr.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/files.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Files.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/FindDirectory.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Finder.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/FinderRegistry.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/FixMath.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/float.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fnmatch.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Folders.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fontconfig/fcfreetype.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fontconfig/fontconfig.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Font.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Fonts.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fp.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fpieee.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/frame/log.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/frame/req.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/freetype/freetype.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/freetype/ftadvanc.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/freetype/ftbitmap.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/freetype/ftcache.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/freetype/ftfntfmt.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/freetype/ftglyph.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/freetype/ftlcdfil.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/freetype/ftoutln.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/freetype/ftsizes.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/freetype/ftsynth.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/freetype/ftxf86.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/freetype.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/freetype/t1tables.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/freetype/ttnameid.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/freetype/tttables.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fribidi/fribidi.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/FSp_fopen.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fstream: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fstream.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ft2build.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ftadvanc.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ftbitmap.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ftcache.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ftfntfmt.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ftglyph.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ftlcdfil.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ftoutln.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fts.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ftsizes.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ftsynth.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ftxf86.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fusion/arena.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fusion/build.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fusion/call.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fusion/conf.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fusion/fusion.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fusion/fusion_internal.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fusion/hash.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fusion/lock.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fusion/object.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fusion/property.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fusion/protocol.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fusion/reactor.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fusion/ref.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fusion/shmalloc.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fusion/shm/pool.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fusion/shm/shm.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fusion/shm/shm_internal.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fusion/types.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/fusion/vector.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/gconf/gconf-client.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Gdiplus.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/gdk/gdkdirectfb.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/gdk/gdk.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/gdk/gdkkeysyms.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/gdk/gdkprivate.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/gdk/gdkwayland.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/gdk-pixbuf/gdk-pixbuf.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Gestalt.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/getopt.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/gio/gio.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/glibconfig.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/glib.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/glib-object.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/glob.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/gmodule.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/gnome.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/gnu/libc-version.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/gps.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/grp.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/gssapi_generic.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/gssapi/gssapi_generic.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/gssapi/gssapi.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/gssapi.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/gst/app/gstappsink.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/gst/app/gstappsrc.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/gst/gst.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/gst/video/video.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/gtk/gtk.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/gtk/gtkunixprint.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/hasht.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/HIToolbox/HIToolbox.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/hlink.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ia64/sys/inline.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Icons.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/iconv.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ieeefp.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ifaddrs.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/image.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/imagehlp.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/imm.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/initguid.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/initializer_list: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/InterfaceDefs.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/InternetConfig.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/IntlResources.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ints.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/intshcut.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/inttypes.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/iodef.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/io.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/IOKit/IOKitLib.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/IOKit/IOMessage.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/IOKit/pwr_mgt/IOPMLib.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/iomanip: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/iostream.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/jar-ds.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/jarfile.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/jar.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/JavaControl.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/JavaEmbedding/JavaControl.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/JavaVM/jni.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/JManager.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/JNIEnvTests.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/jni.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/JVMManagerTests.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Kerberos/Kerberos.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/kernel/image.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/kernel/OS.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/key.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/keyhi.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/keyt.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/keythi.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/kvm.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LAction.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/langinfo.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LApplication.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LArray.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LArrayIterator.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LAttachable.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LAttachment.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LaunchServices.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/lber.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LBroadcaster.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LButton.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/lcache.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LCaption.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LCheckBox.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LCicnButton.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LClipboard.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LCommander.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LComparator.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LControl.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ldap.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ldaplog.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ldappr.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ldap_ssl.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LDataStream.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ldfcn.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LDialogBox.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ldif.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LDocApplication.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LDocument.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LDragAndDrop.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LDragTask.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LEditField.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LEditText.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LEventDispatcher.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LFile.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LFileStream.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LFileTypeList.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LFocusBox.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LGrafPortView.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LHandleStream.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/libc_r.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/libelf.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/libelf/libelf.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/libgen.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/libgnome/gnome-url.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/libgnome/libgnome.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/libgnomeui/gnome-icon-lookup.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/libgnomeui/gnome-icon-theme.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/libgnomeui/gnome-ui-init.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/libutil.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/limits.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/link.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/linux/ioprio.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/linux/kernel.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/linux/limits.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/linux/rtc.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/linux/version.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/List.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Lists.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LListBox.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LListener.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LMenuBar.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LMenu.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LModelDirector.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LModelObject.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LModelProperty.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/loader.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/locale: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/locale.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LOffscreenView.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/logkeys.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/logstrng.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Looper.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LowMem.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LPane.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LPeriodical.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LPicture.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LPlaceHolder.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LPrintout.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LProgressBar.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LPushButton.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LRadioGroup.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LRadioGroupView.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LRunArray.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LScroller.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LSharable.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LSingleDoc.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LStaticText.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LStdControl.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LStream.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LString.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LTabGroup.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LTabGroupView.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LTableArrayStorage.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LTableMonoGeometry.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LTableSingleSelector.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LTableView.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LTextEditView.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LTextTableView.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LUndoer.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LVariableArray.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LView.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/LWindow.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/m68881.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/MacErrors.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/MacHeadersCarbon.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/machine/ansi.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/machine/builtins.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/machine/clock.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/machine/endian.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/machine/frame.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/machine/inline.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/machine/limits.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/machine/signal.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/machine/trap.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/mach/mach_host.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/mach/mach_init.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/mach/mach_interface.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/mach/mach_port.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/mach-o/dyld.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/MacLocales.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/MacMemory.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/MacTCP.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/MacTypes.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/MacWindows.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/malloc.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/malloc_np.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/mapicode.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/mapidefs.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/mapiguid.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/mapi.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/mapitags.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/mapiutil.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/mapix.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Math64.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/math.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/mbstring.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/mem.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/memory.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Memory.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/MenuBar.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Menu.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Menus.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Message.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Mime.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/MixedMode.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/mlang.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/mmsystem.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/model.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Movies.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/mpw/errno.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/mshtmhst.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/mshtml.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/mswsock.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Multiprocessing.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/mutex.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Navigation.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ncompat.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ncurses.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/netCore.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/netdb.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/net/if.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/netinet/in.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/netinet/in_systm.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/netinet/tcp.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/newexe.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/new.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/nl_types.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/NodeInfo.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/nspr.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/nssb64.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/nssb64t.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/nssbase.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/nssbaset.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/nssck.api: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/nssckbi.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/nssckepv.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/nssckft.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/nssckfwc.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/nssckfw.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/nssckfwt.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/nssckg.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/nssckmdt.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/nssckt.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/nss.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/nssilckt.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/nssilock.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/nsslocks.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/nssrwlk.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/nssrwlkt.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/nssutil.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/nsswitch.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/objbase.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/objidl.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Objsafe.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ocsp.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ocspt.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ojiapitests.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ole2.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/oleidl.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/OpenGL/OpenGL.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/OpenTptInternet.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/OpenTransport.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/OS.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/osreldate.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/OSUtils.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/p12.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/p12plcy.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/p12t.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Packages.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Palettes.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/PALM_CMN.H: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pango/pango-break.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pango/pangocairo.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pango/pangofc-decoder.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pango/pangofc-font.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pango/pangofc-fontmap.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pango/pango-fontmap.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pango/pango.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pango/pango-modules.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pango/pango-utils.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pango/pangoxft.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pascal.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Patches.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Path.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pcfs/pc_dir.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Pgenerr.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/PGenErr.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Ph.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pixman.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pk11func.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pk11pqg.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pk11priv.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pk11pub.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pk11sdr.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pkcs11f.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pkcs11.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pkcs11n.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pkcs11p.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pkcs11t.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pkcs11u.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pkcs12.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pkcs12t.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pkcs7t.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/plarena.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/plarenas.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/plbase64.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/plerror.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/plgetopt.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/plhash.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/plstr.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/PLStringFuncs.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/PMApplication.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pmddim.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/poll.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Polygon.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/portable.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/port.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/portreg.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Power.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/PP_ClassHeaders.cp: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/PP_Constants.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/PPCToolbox.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/PP_DebugHeaders.cp: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/PP_KeyCodes.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/PP_Macros.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/PP_Messages.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/PP_Prefix.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/PP_Resources.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/PP_Types.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pratom.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prbit.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prclist.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prcmon.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prcountr.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prcpucfg.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prcvar.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prdtoa.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/preenc.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prenv.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prerr.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prerror.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prinet.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prinit.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prinrval.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Printing.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Print/PMPrintingDialogExtensions.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prio.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pripcsem.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/private: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prlink.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prlock.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prlog.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prlong.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prmem.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prmon.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prmwait.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prnetdb.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Processes.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/process.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Process.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prolock.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/proto/dos.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/proto/exec.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prpdce.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prprf.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prproces.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prrng.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prrwlock.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prshma.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prshm.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prsystem.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prthread.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prtime.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prtpool.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prtrace.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prtypes.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prvrsion.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/prwin16.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/psap.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Pt.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pthread.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pthread_np.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pulse/pulseaudio.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/pwd.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Python.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/QDOffscreen.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/queue: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Quickdraw.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/QuickDraw.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/QuickTimeComponents.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/quipu/attr.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/regex.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Region.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/resolv.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Resources.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Retrace.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/rld_interface.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Roster.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/rpc.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/rpcproxy.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/rpc/types.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sane/sane.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sane/sanei.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sane/saneopts.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sanitizer/asan_interface.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sched.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Scrap.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Screen.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Script.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ScrollBar.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/secasn1.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/secasn1t.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/seccomon.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/secder.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/secdert.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/secdig.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/secdigt.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/secerr.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sec.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sechash.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/secitem.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/secmime.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/secmod.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/secmodt.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/secoid.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/secoidt.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/secpkcs5.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/secpkcs7.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/secport.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/secrng.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/security.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/secutil.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/semaphore.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/servprov.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/setjmp.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/SFNTLayoutTypes.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/SFNTTypes.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sha1.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/share.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/shellapi.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/shlguid.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/shlobj.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/shsign.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sigcontext.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/signal.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/SimpleGameSound.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/SIOUX.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/size_t.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/smime.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sndio.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/someincludefile.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/soundcard.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Sound.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/soundtouch/SoundTouchFactory.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/soundtouch/SoundTouch.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/spawn.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sqlite3.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sslerr.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ssl.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sslproto.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sslt.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sstream: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/StandardFile.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/starlet.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/stat.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/statreg.cpp: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/statreg.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/stdarg.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/stdbool.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/stddef.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/stdint.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/stdio.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/stdlib.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/storage/FindDirectory.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/StorageKit.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/StringCompare.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/string.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/String.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/strings.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Strings.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/StringView.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/stropts.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/strstrea.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/structs.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/stsdef.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/SupportDefs.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/support/String.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/support/SupportDefs.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/support/TLS.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/svrcore.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/symconst.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sym.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/synch.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/syncmgr.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/atomic_op.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/bitypes.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/byteorder.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/syscall.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/cdefs.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/cfgodm.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/elf.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/endian.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/epoll.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/errno.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/eventfd.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/fault.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/fcntl.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/file.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/filio.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/frame.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/immu.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/inotify.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/inttypes.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/ioccom.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/ioctl.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/ipc.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/klog.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/ldr.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/link.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/locking.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/syslog.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/lwp.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/machine.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/mman.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/mmu.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/mount.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/mpctl.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/msg.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/param.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/pda.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/poll.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/ppc.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/prctl.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/priv.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/procfs.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/pstat.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/ptrace.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/queue.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/quota.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/reboot.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/reg.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/regset.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/resource.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/sched.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/select.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/sem.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/sendfile.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/shm.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/siginfo.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/signal.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/socket.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/sockio.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/sparc/frame.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/stack.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/statfs.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/stat.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/statvfs.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/syscall.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/sysctl.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/sysinfo.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/sysmacros.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/sysmp.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/syssgi.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/systeminfo.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/system_properties.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/thr.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/timeb.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/time.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/times.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/ttycom.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/types.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/ucontext.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/uio.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/un.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/unistd.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/user.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/utsname.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/vfs.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/wait.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/t1tables.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/tables.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/TArray.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/TArrayIterator.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/task.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/tchar.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/TCHAR.H: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/termios.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/TextCommon.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/TextEdit.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/TextEncodingConverter.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/TextServices.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/TextUtils.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/TextView.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/th/PCR_Th.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/thread.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ThreadManagerTests.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Threads.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/time.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Timer.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/tlhelp32.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ToolUtils.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/tr1/functional: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/trace.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Traps.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ttnameid.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/tttables.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/typeinfo: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/types.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Types.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UAppleEventsMgr.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UAttachments.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ucontext.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/uconv.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UCursor.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UDebugging.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UDesktop.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UDrawingState.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UDrawingUtils.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UEnvironment.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UEventMgr.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UException.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UExtractFromAEDesc.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UGWorld.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UKeyFilters.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ulocks.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/ulserrno.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UMemoryMgr.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UModalDialogs.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UNavServicesDialogs.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UnicodeBlockObjects.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UnicodeConverter.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UnicodeUtilities.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/unidef.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/unikbd.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/unistd.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/unix.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/unixio.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/unknwn.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/unwind.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UPrinting.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UQuickTime.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UReanimator.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/URegions.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/URegistrar.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UResourceMgr.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/urlhist.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/urlmon.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UScrap.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UScreenPort.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UTCUtils.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UTETextAction.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UTEViewTextAction.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UTextEdit.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UTextTraits.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/utilmodt.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/utilpars.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/utilparst.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/utilrename.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/utime.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/UWindows.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/values.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/varargs.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/vcclr.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/View.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Volume.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/wab.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/wait.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/wchar.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/wctype.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/winbase.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/win/compobj.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/windef.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Window.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/windows.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Windows.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/windowsx.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Wininet.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/winnls.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/winperf.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/winreg.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/Winreg.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/winsock2.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/winsock.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/winspool.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/winsvc.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/winuser.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/winver.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/wmem.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/workbench/startup.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/wtypes.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/wx/image.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/wx/listctrl.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/wx/log.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/wx/toolbar.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/wx/wx.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/wx/xrc/xmlres.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/xlocale.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/zmouse.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/gdk/gdkx.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/gtk/gtkx.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/X11/cursorfont.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/X11/extensions/Print.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/X11/extensions/scrnsaver.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/X11/extensions/shape.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/X11/extensions/Xcomposite.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/X11/extensions/Xdamage.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/X11/extensions/Xfixes.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/X11/extensions/Xrender.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/X11/extensions/XShm.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/X11/ImUtil.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/X11/Intrinsic.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/X11/keysymdef.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/X11/keysym.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/X11/Shell.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/X11/StringDefs.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/X11/Xatom.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/X11/Xft/Xft.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/X11/Xfuncproto.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/X11/X.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/X11/XKBlib.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/X11/Xlib.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/X11/Xlibint.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/X11/Xlib-xcb.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/X11/Xlocale.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/X11/Xos.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/X11/Xutil.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/xcb/shm.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/xcb/xcb.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/libavcodec/avfft.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/event.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/xkbcommon/xkbcommon.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/wayland-client.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/wayland-egl.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/wayland-util.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
$(TOPOBJDIR)/dist/system_wrappers/sys/auxv.h: $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.pp
$(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub: /worker/build/config/make-system-wrappers.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/config/make-system-wrappers.py gen_wrappers $(TOPOBJDIR)/config/system-header.sentinel $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.pp $(TOPOBJDIR)/config/$(MDDEPDIR)/system-header.sentinel.stub /worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers new algorithm atomic cassert climits cmath condition_variable cstdarg cstdio cstdlib cstring cwchar deque functional ios iosfwd iostream istream iterator limits list map memory mutex ostream regex set shared_mutex stack string thread tuple type_traits unordered_map unordered_set utility vector xutility A4Stuff.h activscp.h AEDataModel.h AEObjects.h AEPackObject.h AERegistry.h AEUtils.h afxcmn.h afxcoll.h afxcview.h afxdisp.h afxdtctl.h afxext.h afxmt.h afxpriv.h afxtempl.h afxwin.h Aliases.h all.h alloca.h alloc.h alsa/asoundlib.h ansi_parms.h a.out.h app/Cursor.h Appearance.h AppFileInfo.h AppKit.h AppleEvents.h Application.h app/Message.h app/MessageRunner.h arpa/inet.h arpa/nameser.h array asm/page.h asm/sigcontext.h asm/signal.h ASRegistry.h assert.h atk/atk.h atlcom.h atlconv.h atlctl.cpp atlctl.h ATLCTL.H atlhost.h atlimpl.cpp atlwin.cpp atomic.h ATSTypes.h ATSUnicode.h Balloons.h base64.h base/pblock.h base/PCR_Base.h base/session.h basetyps.h be/app/Application.h Beep.h be/kernel/image.h be/kernel/OS.h bfd.h Bitmap.h bitset blapi.h blapit.h bsd/libc.h bsd/syscall.h bstring.h builtin.h Button.h byteswap.h cairo-atsui.h cairo-beos.h cairo-directfb.h cairo-ft.h cairo-glitz.h cairo-gobject.h cairo.h cairo-pdf.h cairo-ps.h cairo-qpainter.h cairo-qt.h cairo-quartz.h cairo-tee.h cairo-win32.h cairo-xlib.h cairo-xlib-xrender.h callconv.h Carbon/Carbon.h CarbonEvents.h Carbon.h c_asm.h cctype cderr.h cerrno certdb.h cert.h certt.h CFBase.h CFBundle.h CFData.h CFDictionary.h cf.h CFNumber.h CFPlugIn.h CFPreferences.h CFString.h CFURL.h CGAffineTransform.h CheckBox.h ciferfam.h Clipboard.h cmmf.h cmmft.h cmplrs/stsupport.h cms.h cmsreclist.h cmst.h Cocoa/Cocoa.h CodeFragments.h comdef.h commctrl.h COMMCTRL.H commdlg.h compat.h complex condapi.h ConditionalMacros.h conio.h console.h ControlDefinitions.h Controls.h CoreFoundation/CoreFoundation.h CoreServices/CoreServices.h CPalmRec.cpp Cpalmrec.h CPCatgry.cpp CPDbBMgr.h CPString.cpp CPString.h crmf.h crmft.h crtdbg.h crt_externs.h crypt.h cryptohi.h cryptoht.h cstddef ctime ctype.h curl/curl.h curl/easy.h curses.h cxxabi.h DateTimeUtils.h dbus/dbus-glib.h dbus/dbus-glib-lowlevel.h dbus/dbus.h ddeml.h Debug.h dem.h descrip.h Devices.h dfb_types.h dfiff.h dgiff.h Dialogs.h direct/build.h direct/clock.h direct/conf.h direct/debug.h direct/direct.h directfb_keyboard.h directfb_strings.h direct.h direct/hash.h direct/interface.h direct/interface_implementation.h direct/list.h direct/log.h direct/memcpy.h direct/mem.h direct/messages.h direct/modules.h direct/serial.h direct/signals.h direct/stream.h direct/system.h direct/thread.h direct/trace.h direct/tree.h direct/types.h direct/utf8.h direct/util.h dirent.h DiskInit.h dlfcn.h dlgs.h dl.h docobj.h dos/dosextens.h dos.h Drag.h DriverServices.h DriverSynchronization.h DropInPanel.h dvidef.h ecl-exp.h elf.h endian.h Entry.h err.h errno.h Errors.h Events.h exception exdisp.h ExDisp.h exe386.h execinfo.h extras.h fcntl.h features.h fenv.h ffi.h fibdef.h File.h filehdr.h files.h Files.h FindDirectory.h Finder.h FinderRegistry.h FixMath.h float.h fnmatch.h Folders.h fontconfig/fcfreetype.h fontconfig/fontconfig.h Font.h Fonts.h fp.h fpieee.h frame/log.h frame/req.h freetype/freetype.h freetype/ftadvanc.h freetype/ftbitmap.h freetype/ftcache.h freetype/ftfntfmt.h freetype/ftglyph.h freetype/ftlcdfil.h freetype/ftoutln.h freetype/ftsizes.h freetype/ftsynth.h freetype/ftxf86.h freetype.h freetype/t1tables.h freetype/ttnameid.h freetype/tttables.h fribidi/fribidi.h FSp_fopen.h fstream fstream.h ft2build.h ftadvanc.h ftbitmap.h ftcache.h ftfntfmt.h ftglyph.h ftlcdfil.h ftoutln.h fts.h ftsizes.h ftsynth.h ftxf86.h fusion/arena.h fusion/build.h fusion/call.h fusion/conf.h fusion/fusion.h fusion/fusion_internal.h fusion/hash.h fusion/lock.h fusion/object.h fusion/property.h fusion/protocol.h fusion/reactor.h fusion/ref.h fusion/shmalloc.h fusion/shm/pool.h fusion/shm/shm.h fusion/shm/shm_internal.h fusion/types.h fusion/vector.h gconf/gconf-client.h Gdiplus.h gdk/gdkdirectfb.h gdk/gdk.h gdk/gdkkeysyms.h gdk/gdkprivate.h gdk/gdkwayland.h gdk-pixbuf/gdk-pixbuf.h Gestalt.h getopt.h gio/gio.h glibconfig.h glib.h glib-object.h glob.h gmodule.h gnome.h gnu/libc-version.h gps.h grp.h gssapi_generic.h gssapi/gssapi_generic.h gssapi/gssapi.h gssapi.h gst/app/gstappsink.h gst/app/gstappsrc.h gst/gst.h gst/video/video.h gtk/gtk.h gtk/gtkunixprint.h hasht.h HIToolbox/HIToolbox.h hlink.h ia64/sys/inline.h Icons.h iconv.h ieeefp.h ifaddrs.h image.h imagehlp.h imm.h initguid.h initializer_list InterfaceDefs.h InternetConfig.h IntlResources.h ints.h intshcut.h inttypes.h iodef.h io.h IOKit/IOKitLib.h IOKit/IOMessage.h IOKit/pwr_mgt/IOPMLib.h iomanip iostream.h jar-ds.h jarfile.h jar.h JavaControl.h JavaEmbedding/JavaControl.h JavaVM/jni.h JManager.h JNIEnvTests.h jni.h JVMManagerTests.h Kerberos/Kerberos.h kernel/image.h kernel/OS.h key.h keyhi.h keyt.h keythi.h kvm.h LAction.h langinfo.h LApplication.h LArray.h LArrayIterator.h LAttachable.h LAttachment.h LaunchServices.h lber.h LBroadcaster.h LButton.h lcache.h LCaption.h LCheckBox.h LCicnButton.h LClipboard.h LCommander.h LComparator.h LControl.h ldap.h ldaplog.h ldappr.h ldap_ssl.h LDataStream.h ldfcn.h LDialogBox.h ldif.h LDocApplication.h LDocument.h LDragAndDrop.h LDragTask.h LEditField.h LEditText.h LEventDispatcher.h LFile.h LFileStream.h LFileTypeList.h LFocusBox.h LGrafPortView.h LHandleStream.h libc_r.h libelf.h libelf/libelf.h libgen.h libgnome/gnome-url.h libgnome/libgnome.h libgnomeui/gnome-icon-lookup.h libgnomeui/gnome-icon-theme.h libgnomeui/gnome-ui-init.h libutil.h limits.h link.h linux/ioprio.h linux/kernel.h linux/limits.h linux/rtc.h linux/version.h List.h Lists.h LListBox.h LListener.h LMenuBar.h LMenu.h LModelDirector.h LModelObject.h LModelProperty.h loader.h locale locale.h LOffscreenView.h logkeys.h logstrng.h Looper.h LowMem.h LPane.h LPeriodical.h LPicture.h LPlaceHolder.h LPrintout.h LProgressBar.h LPushButton.h LRadioGroup.h LRadioGroupView.h LRunArray.h LScroller.h LSharable.h LSingleDoc.h LStaticText.h LStdControl.h LStream.h LString.h LTabGroup.h LTabGroupView.h LTableArrayStorage.h LTableMonoGeometry.h LTableSingleSelector.h LTableView.h LTextEditView.h LTextTableView.h LUndoer.h LVariableArray.h LView.h LWindow.h m68881.h MacErrors.h MacHeadersCarbon.h machine/ansi.h machine/builtins.h machine/clock.h machine/endian.h machine/frame.h machine/inline.h machine/limits.h machine/signal.h machine/trap.h mach/mach_host.h mach/mach_init.h mach/mach_interface.h mach/mach_port.h mach-o/dyld.h MacLocales.h MacMemory.h MacTCP.h MacTypes.h MacWindows.h malloc.h malloc_np.h mapicode.h mapidefs.h mapiguid.h mapi.h mapitags.h mapiutil.h mapix.h Math64.h math.h mbstring.h mem.h memory.h Memory.h MenuBar.h Menu.h Menus.h Message.h Mime.h MixedMode.h mlang.h mmsystem.h model.h Movies.h mpw/errno.h mshtmhst.h mshtml.h mswsock.h Multiprocessing.h mutex.h Navigation.h ncompat.h ncurses.h netCore.h netdb.h net/if.h netinet/in.h netinet/in_systm.h netinet/tcp.h newexe.h new.h nl_types.h NodeInfo.h nspr.h nssb64.h nssb64t.h nssbase.h nssbaset.h nssck.api nssckbi.h nssckepv.h nssckft.h nssckfwc.h nssckfw.h nssckfwt.h nssckg.h nssckmdt.h nssckt.h nss.h nssilckt.h nssilock.h nsslocks.h nssrwlk.h nssrwlkt.h nssutil.h nsswitch.h objbase.h objidl.h Objsafe.h ocsp.h ocspt.h ojiapitests.h ole2.h oleidl.h OpenGL/OpenGL.h OpenTptInternet.h OpenTransport.h OS.h osreldate.h OSUtils.h p12.h p12plcy.h p12t.h Packages.h Palettes.h PALM_CMN.H pango/pango-break.h pango/pangocairo.h pango/pangofc-decoder.h pango/pangofc-font.h pango/pangofc-fontmap.h pango/pango-fontmap.h pango/pango.h pango/pango-modules.h pango/pango-utils.h pango/pangoxft.h pascal.h Patches.h Path.h pcfs/pc_dir.h Pgenerr.h PGenErr.h Ph.h pixman.h pk11func.h pk11pqg.h pk11priv.h pk11pub.h pk11sdr.h pkcs11f.h pkcs11.h pkcs11n.h pkcs11p.h pkcs11t.h pkcs11u.h pkcs12.h pkcs12t.h pkcs7t.h plarena.h plarenas.h plbase64.h plerror.h plgetopt.h plhash.h plstr.h PLStringFuncs.h PMApplication.h pmddim.h poll.h Polygon.h portable.h port.h portreg.h Power.h PP_ClassHeaders.cp PP_Constants.h PPCToolbox.h PP_DebugHeaders.cp PP_KeyCodes.h PP_Macros.h PP_Messages.h PP_Prefix.h PP_Resources.h PP_Types.h pratom.h prbit.h prclist.h prcmon.h prcountr.h prcpucfg.h prcvar.h prdtoa.h preenc.h prenv.h prerr.h prerror.h prinet.h prinit.h prinrval.h Printing.h Print/PMPrintingDialogExtensions.h prio.h pripcsem.h private prlink.h prlock.h prlog.h prlong.h prmem.h prmon.h prmwait.h prnetdb.h Processes.h process.h Process.h prolock.h proto/dos.h proto/exec.h prpdce.h prprf.h prproces.h prrng.h prrwlock.h prshma.h prshm.h prsystem.h prthread.h prtime.h prtpool.h prtrace.h prtypes.h prvrsion.h prwin16.h psap.h Pt.h pthread.h pthread_np.h pulse/pulseaudio.h pwd.h Python.h QDOffscreen.h queue Quickdraw.h QuickDraw.h QuickTimeComponents.h quipu/attr.h regex.h Region.h resolv.h Resources.h Retrace.h rld_interface.h Roster.h rpc.h rpcproxy.h rpc/types.h sane/sane.h sane/sanei.h sane/saneopts.h sanitizer/asan_interface.h sched.h Scrap.h Screen.h Script.h ScrollBar.h secasn1.h secasn1t.h seccomon.h secder.h secdert.h secdig.h secdigt.h secerr.h sec.h sechash.h secitem.h secmime.h secmod.h secmodt.h secoid.h secoidt.h secpkcs5.h secpkcs7.h secport.h secrng.h security.h secutil.h semaphore.h servprov.h setjmp.h SFNTLayoutTypes.h SFNTTypes.h sha1.h share.h shellapi.h shlguid.h shlobj.h shsign.h sigcontext.h signal.h SimpleGameSound.h SIOUX.h size_t.h smime.h sndio.h someincludefile.h soundcard.h Sound.h soundtouch/SoundTouchFactory.h soundtouch/SoundTouch.h spawn.h sqlite3.h sslerr.h ssl.h sslproto.h sslt.h sstream StandardFile.h starlet.h stat.h statreg.cpp statreg.h stdarg.h stdbool.h stddef.h stdint.h stdio.h stdlib.h storage/FindDirectory.h StorageKit.h StringCompare.h string.h String.h strings.h Strings.h StringView.h stropts.h strstrea.h structs.h stsdef.h SupportDefs.h support/String.h support/SupportDefs.h support/TLS.h svrcore.h symconst.h sym.h synch.h syncmgr.h sys/atomic_op.h sys/bitypes.h sys/byteorder.h syscall.h sys/cdefs.h sys/cfgodm.h sys/elf.h sys/endian.h sys/epoll.h sys/errno.h sys/eventfd.h sys/fault.h sys/fcntl.h sys/file.h sys/filio.h sys/frame.h sys/immu.h sys/inotify.h sys/inttypes.h sys/ioccom.h sys/ioctl.h sys/ipc.h sys/klog.h sys/ldr.h sys/link.h sys/locking.h syslog.h sys/lwp.h sys/machine.h sys/mman.h sys/mmu.h sys/mount.h sys/mpctl.h sys/msg.h sys/param.h sys/pda.h sys/poll.h sys/ppc.h sys/prctl.h sys/priv.h sys/procfs.h sys/pstat.h sys/ptrace.h sys/queue.h sys/quota.h sys/reboot.h sys/reg.h sys/regset.h sys/resource.h sys/sched.h sys/select.h sys/sem.h sys/sendfile.h sys/shm.h sys/siginfo.h sys/signal.h sys/socket.h sys/sockio.h sys/sparc/frame.h sys/stack.h sys/statfs.h sys/stat.h sys/statvfs.h sys/syscall.h sys/sysctl.h sys/sysinfo.h sys/sysmacros.h sys/sysmp.h sys/syssgi.h sys/systeminfo.h sys/system_properties.h sys/thr.h sys/timeb.h sys/time.h sys/times.h sys/ttycom.h sys/types.h sys/ucontext.h sys/uio.h sys/un.h sys/unistd.h sys/user.h sys/utsname.h sys/vfs.h sys/wait.h t1tables.h tables.h TArray.h TArrayIterator.h task.h tchar.h TCHAR.H termios.h TextCommon.h TextEdit.h TextEncodingConverter.h TextServices.h TextUtils.h TextView.h th/PCR_Th.h thread.h ThreadManagerTests.h Threads.h time.h Timer.h tlhelp32.h ToolUtils.h tr1/functional trace.h Traps.h ttnameid.h tttables.h typeinfo types.h Types.h UAppleEventsMgr.h UAttachments.h ucontext.h uconv.h UCursor.h UDebugging.h UDesktop.h UDrawingState.h UDrawingUtils.h UEnvironment.h UEventMgr.h UException.h UExtractFromAEDesc.h UGWorld.h UKeyFilters.h ulocks.h ulserrno.h UMemoryMgr.h UModalDialogs.h UNavServicesDialogs.h UnicodeBlockObjects.h UnicodeConverter.h UnicodeUtilities.h unidef.h unikbd.h unistd.h unix.h unixio.h unknwn.h unwind.h UPrinting.h UQuickTime.h UReanimator.h URegions.h URegistrar.h UResourceMgr.h urlhist.h urlmon.h UScrap.h UScreenPort.h UTCUtils.h UTETextAction.h UTEViewTextAction.h UTextEdit.h UTextTraits.h utilmodt.h utilpars.h utilparst.h utilrename.h utime.h UWindows.h values.h varargs.h vcclr.h View.h Volume.h wab.h wait.h wchar.h wctype.h winbase.h win/compobj.h windef.h Window.h windows.h Windows.h windowsx.h Wininet.h winnls.h winperf.h winreg.h Winreg.h winsock2.h winsock.h winspool.h winsvc.h winuser.h winver.h wmem.h workbench/startup.h wtypes.h wx/image.h wx/listctrl.h wx/log.h wx/toolbar.h wx/wx.h wx/xrc/xmlres.h xlocale.h zmouse.h gdk/gdkx.h gtk/gtkx.h X11/cursorfont.h X11/extensions/Print.h X11/extensions/scrnsaver.h X11/extensions/shape.h X11/extensions/Xcomposite.h X11/extensions/Xdamage.h X11/extensions/Xfixes.h X11/extensions/Xrender.h X11/extensions/XShm.h X11/ImUtil.h X11/Intrinsic.h X11/keysymdef.h X11/keysym.h X11/Shell.h X11/StringDefs.h X11/Xatom.h X11/Xft/Xft.h X11/Xfuncproto.h X11/X.h X11/XKBlib.h X11/Xlib.h X11/Xlibint.h X11/Xlib-xcb.h X11/Xlocale.h X11/Xos.h X11/Xutil.h xcb/shm.h xcb/xcb.h libavcodec/avfft.h sys/event.h xkbcommon/xkbcommon.h wayland-client.h wayland-egl.h wayland-util.h sys/auxv.h)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/config/$(MDDEPDIR)/cbindgen-metadata.json.stub
$(TOPOBJDIR)/config/cbindgen-metadata.json: $(TOPOBJDIR)/config/$(MDDEPDIR)/cbindgen-metadata.json.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/config/$(MDDEPDIR)/cbindgen-metadata.json.pp
$(TOPOBJDIR)/config/$(MDDEPDIR)/cbindgen-metadata.json.stub: /worker/build/build/RunCbindgen.py $(TOPOBJDIR)/.cargo/config
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate_metadata $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPOBJDIR)/config/$(MDDEPDIR)/cbindgen-metadata.json.pp $(TOPOBJDIR)/config/$(MDDEPDIR)/cbindgen-metadata.json.stub $(TOPOBJDIR)/.cargo/config)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/js/src/$(MDDEPDIR)/js-confdefs.h.stub
$(TOPOBJDIR)/js/src/js-confdefs.h: $(TOPOBJDIR)/js/src/$(MDDEPDIR)/js-confdefs.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/js/src/$(MDDEPDIR)/js-confdefs.h.pp
$(TOPOBJDIR)/js/src/$(MDDEPDIR)/js-confdefs.h.stub: /worker/build/python/mozbuild/mozbuild/action/process_define_files.py $(TOPSRCDIR)/js/src/js-confdefs.h.in
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/process_define_files.py process_define_file $(TOPOBJDIR)/js/src/js-confdefs.h $(TOPOBJDIR)/js/src/$(MDDEPDIR)/js-confdefs.h.pp $(TOPOBJDIR)/js/src/$(MDDEPDIR)/js-confdefs.h.stub $(TOPSRCDIR)/js/src/js-confdefs.h.in)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/js/src/$(MDDEPDIR)/js-config.h.stub
$(TOPOBJDIR)/js/src/js-config.h: $(TOPOBJDIR)/js/src/$(MDDEPDIR)/js-config.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/js/src/$(MDDEPDIR)/js-config.h.pp
$(TOPOBJDIR)/js/src/$(MDDEPDIR)/js-config.h.stub: /worker/build/python/mozbuild/mozbuild/action/process_define_files.py $(TOPSRCDIR)/js/src/js-config.h.in
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/process_define_files.py process_define_file $(TOPOBJDIR)/js/src/js-config.h $(TOPOBJDIR)/js/src/$(MDDEPDIR)/js-config.h.pp $(TOPOBJDIR)/js/src/$(MDDEPDIR)/js-config.h.stub $(TOPSRCDIR)/js/src/js-config.h.in)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/js/src/$(MDDEPDIR)/selfhosted.out.h.stub
$(TOPOBJDIR)/js/src/selfhosted.out.h: $(TOPOBJDIR)/js/src/$(MDDEPDIR)/selfhosted.out.h.stub ;
$(TOPOBJDIR)/js/src/selfhosted.js: $(TOPOBJDIR)/js/src/$(MDDEPDIR)/selfhosted.out.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/js/src/$(MDDEPDIR)/selfhosted.out.h.pp
$(TOPOBJDIR)/js/src/$(MDDEPDIR)/selfhosted.out.h.stub: /worker/build/js/src/builtin/embedjs.py $(TOPSRCDIR)/js/public/friend/ErrorNumbers.msg $(TOPSRCDIR)/js/src/builtin/TypedArrayConstants.h $(TOPSRCDIR)/js/src/builtin/SelfHostingDefines.h $(TOPSRCDIR)/js/src/builtin/Utilities.js $(TOPSRCDIR)/js/src/builtin/Array.js $(TOPSRCDIR)/js/src/builtin/AsyncFunction.js $(TOPSRCDIR)/js/src/builtin/AsyncIteration.js $(TOPSRCDIR)/js/src/builtin/BigInt.js $(TOPSRCDIR)/js/src/builtin/Date.js $(TOPSRCDIR)/js/src/builtin/Error.js $(TOPSRCDIR)/js/src/builtin/Function.js $(TOPSRCDIR)/js/src/builtin/Generator.js $(TOPSRCDIR)/js/src/builtin/Iterator.js $(TOPSRCDIR)/js/src/builtin/Map.js $(TOPSRCDIR)/js/src/builtin/Module.js $(TOPSRCDIR)/js/src/builtin/Number.js $(TOPSRCDIR)/js/src/builtin/Object.js $(TOPSRCDIR)/js/src/builtin/Promise.js $(TOPSRCDIR)/js/src/builtin/Reflect.js $(TOPSRCDIR)/js/src/builtin/RegExp.js $(TOPSRCDIR)/js/src/builtin/RegExpGlobalReplaceOpt.h.js $(TOPSRCDIR)/js/src/builtin/RegExpLocalReplaceOpt.h.js $(TOPSRCDIR)/js/src/builtin/String.js $(TOPSRCDIR)/js/src/builtin/Set.js $(TOPSRCDIR)/js/src/builtin/Sorting.js $(TOPSRCDIR)/js/src/builtin/TypedArray.js $(TOPSRCDIR)/js/src/builtin/WeakMap.js $(TOPSRCDIR)/js/src/builtin/WeakSet.js $(TOPSRCDIR)/js/src/builtin/intl/NumberingSystemsGenerated.h $(TOPSRCDIR)/js/src/builtin/intl/Collator.js $(TOPSRCDIR)/js/src/builtin/intl/CommonFunctions.js $(TOPSRCDIR)/js/src/builtin/intl/CurrencyDataGenerated.js $(TOPSRCDIR)/js/src/builtin/intl/DateTimeFormat.js $(TOPSRCDIR)/js/src/builtin/intl/DisplayNames.js $(TOPSRCDIR)/js/src/builtin/intl/IntlObject.js $(TOPSRCDIR)/js/src/builtin/intl/ListFormat.js $(TOPSRCDIR)/js/src/builtin/intl/NumberFormat.js $(TOPSRCDIR)/js/src/builtin/intl/PluralRules.js $(TOPSRCDIR)/js/src/builtin/intl/RelativeTimeFormat.js $(TOPSRCDIR)/js/src/builtin/intl/SanctionedSimpleUnitIdentifiersGenerated.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/js/src/builtin/embedjs.py generate_selfhosted $(TOPOBJDIR)/js/src/selfhosted.out.h $(TOPOBJDIR)/js/src/$(MDDEPDIR)/selfhosted.out.h.pp $(TOPOBJDIR)/js/src/$(MDDEPDIR)/selfhosted.out.h.stub $(TOPSRCDIR)/js/public/friend/ErrorNumbers.msg $(TOPSRCDIR)/js/src/builtin/TypedArrayConstants.h $(TOPSRCDIR)/js/src/builtin/SelfHostingDefines.h $(TOPSRCDIR)/js/src/builtin/Utilities.js $(TOPSRCDIR)/js/src/builtin/Array.js $(TOPSRCDIR)/js/src/builtin/AsyncFunction.js $(TOPSRCDIR)/js/src/builtin/AsyncIteration.js $(TOPSRCDIR)/js/src/builtin/BigInt.js $(TOPSRCDIR)/js/src/builtin/Date.js $(TOPSRCDIR)/js/src/builtin/Error.js $(TOPSRCDIR)/js/src/builtin/Function.js $(TOPSRCDIR)/js/src/builtin/Generator.js $(TOPSRCDIR)/js/src/builtin/Iterator.js $(TOPSRCDIR)/js/src/builtin/Map.js $(TOPSRCDIR)/js/src/builtin/Module.js $(TOPSRCDIR)/js/src/builtin/Number.js $(TOPSRCDIR)/js/src/builtin/Object.js $(TOPSRCDIR)/js/src/builtin/Promise.js $(TOPSRCDIR)/js/src/builtin/Reflect.js $(TOPSRCDIR)/js/src/builtin/RegExp.js $(TOPSRCDIR)/js/src/builtin/RegExpGlobalReplaceOpt.h.js $(TOPSRCDIR)/js/src/builtin/RegExpLocalReplaceOpt.h.js $(TOPSRCDIR)/js/src/builtin/String.js $(TOPSRCDIR)/js/src/builtin/Set.js $(TOPSRCDIR)/js/src/builtin/Sorting.js $(TOPSRCDIR)/js/src/builtin/TypedArray.js $(TOPSRCDIR)/js/src/builtin/WeakMap.js $(TOPSRCDIR)/js/src/builtin/WeakSet.js $(TOPSRCDIR)/js/src/builtin/intl/NumberingSystemsGenerated.h $(TOPSRCDIR)/js/src/builtin/intl/Collator.js $(TOPSRCDIR)/js/src/builtin/intl/CommonFunctions.js $(TOPSRCDIR)/js/src/builtin/intl/CurrencyDataGenerated.js $(TOPSRCDIR)/js/src/builtin/intl/DateTimeFormat.js $(TOPSRCDIR)/js/src/builtin/intl/DisplayNames.js $(TOPSRCDIR)/js/src/builtin/intl/IntlObject.js $(TOPSRCDIR)/js/src/builtin/intl/ListFormat.js $(TOPSRCDIR)/js/src/builtin/intl/NumberFormat.js $(TOPSRCDIR)/js/src/builtin/intl/PluralRules.js $(TOPSRCDIR)/js/src/builtin/intl/RelativeTimeFormat.js $(TOPSRCDIR)/js/src/builtin/intl/SanctionedSimpleUnitIdentifiersGenerated.js)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/js/src/build/$(MDDEPDIR)/spidermonkey_checks.stub
$(TOPOBJDIR)/js/src/build/spidermonkey_checks: $(TOPOBJDIR)/js/src/build/$(MDDEPDIR)/spidermonkey_checks.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/js/src/build/$(MDDEPDIR)/spidermonkey_checks.pp
$(TOPOBJDIR)/js/src/build/$(MDDEPDIR)/spidermonkey_checks.stub: /worker/build/config/run_spidermonkey_checks.py $(TOPOBJDIR)/js/src/build/libjs_static.a $(TOPSRCDIR)/config/check_spidermonkey_style.py $(TOPSRCDIR)/config/check_macroassembler_style.py $(TOPSRCDIR)/config/check_js_opcode.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/config/run_spidermonkey_checks.py main $(TOPOBJDIR)/js/src/build/spidermonkey_checks $(TOPOBJDIR)/js/src/build/$(MDDEPDIR)/spidermonkey_checks.pp $(TOPOBJDIR)/js/src/build/$(MDDEPDIR)/spidermonkey_checks.stub $(TOPOBJDIR)/js/src/build/libjs_static.a $(TOPSRCDIR)/config/check_spidermonkey_style.py $(TOPSRCDIR)/config/check_macroassembler_style.py $(TOPSRCDIR)/config/check_js_opcode.py)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/js/src/frontend/$(MDDEPDIR)/ReservedWordsGenerated.h.stub
$(TOPOBJDIR)/js/src/frontend/ReservedWordsGenerated.h: $(TOPOBJDIR)/js/src/frontend/$(MDDEPDIR)/ReservedWordsGenerated.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/js/src/frontend/$(MDDEPDIR)/ReservedWordsGenerated.h.pp
$(TOPOBJDIR)/js/src/frontend/$(MDDEPDIR)/ReservedWordsGenerated.h.stub: /worker/build/js/src/frontend/GenerateReservedWords.py $(TOPSRCDIR)/js/src/frontend/ReservedWords.h
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/js/src/frontend/GenerateReservedWords.py main $(TOPOBJDIR)/js/src/frontend/ReservedWordsGenerated.h $(TOPOBJDIR)/js/src/frontend/$(MDDEPDIR)/ReservedWordsGenerated.h.pp $(TOPOBJDIR)/js/src/frontend/$(MDDEPDIR)/ReservedWordsGenerated.h.stub $(TOPSRCDIR)/js/src/frontend/ReservedWords.h)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/js/src/gc/$(MDDEPDIR)/StatsPhasesGenerated.h.stub
$(TOPOBJDIR)/js/src/gc/StatsPhasesGenerated.h: $(TOPOBJDIR)/js/src/gc/$(MDDEPDIR)/StatsPhasesGenerated.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/js/src/gc/$(MDDEPDIR)/StatsPhasesGenerated.h.pp
$(TOPOBJDIR)/js/src/gc/$(MDDEPDIR)/StatsPhasesGenerated.h.stub: /worker/build/js/src/gc/GenerateStatsPhases.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/js/src/gc/GenerateStatsPhases.py generateHeader $(TOPOBJDIR)/js/src/gc/StatsPhasesGenerated.h $(TOPOBJDIR)/js/src/gc/$(MDDEPDIR)/StatsPhasesGenerated.h.pp $(TOPOBJDIR)/js/src/gc/$(MDDEPDIR)/StatsPhasesGenerated.h.stub)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/js/src/gc/$(MDDEPDIR)/StatsPhasesGenerated.inc.stub
$(TOPOBJDIR)/js/src/gc/StatsPhasesGenerated.inc: $(TOPOBJDIR)/js/src/gc/$(MDDEPDIR)/StatsPhasesGenerated.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/js/src/gc/$(MDDEPDIR)/StatsPhasesGenerated.inc.pp
$(TOPOBJDIR)/js/src/gc/$(MDDEPDIR)/StatsPhasesGenerated.inc.stub: /worker/build/js/src/gc/GenerateStatsPhases.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/js/src/gc/GenerateStatsPhases.py generateCpp $(TOPOBJDIR)/js/src/gc/StatsPhasesGenerated.inc $(TOPOBJDIR)/js/src/gc/$(MDDEPDIR)/StatsPhasesGenerated.inc.pp $(TOPOBJDIR)/js/src/gc/$(MDDEPDIR)/StatsPhasesGenerated.inc.stub)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/js/src/jit/$(MDDEPDIR)/MOpcodesGenerated.h.stub
$(TOPOBJDIR)/js/src/jit/MOpcodesGenerated.h: $(TOPOBJDIR)/js/src/jit/$(MDDEPDIR)/MOpcodesGenerated.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/js/src/jit/$(MDDEPDIR)/MOpcodesGenerated.h.pp
$(TOPOBJDIR)/js/src/jit/$(MDDEPDIR)/MOpcodesGenerated.h.stub: /worker/build/js/src/jit/GenerateOpcodeFiles.py $(TOPSRCDIR)/js/src/jit/MIR.h
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/js/src/jit/GenerateOpcodeFiles.py generate_mir_header $(TOPOBJDIR)/js/src/jit/MOpcodesGenerated.h $(TOPOBJDIR)/js/src/jit/$(MDDEPDIR)/MOpcodesGenerated.h.pp $(TOPOBJDIR)/js/src/jit/$(MDDEPDIR)/MOpcodesGenerated.h.stub $(TOPSRCDIR)/js/src/jit/MIR.h)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/js/src/jit/$(MDDEPDIR)/LOpcodesGenerated.h.stub
$(TOPOBJDIR)/js/src/jit/LOpcodesGenerated.h: $(TOPOBJDIR)/js/src/jit/$(MDDEPDIR)/LOpcodesGenerated.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/js/src/jit/$(MDDEPDIR)/LOpcodesGenerated.h.pp
$(TOPOBJDIR)/js/src/jit/$(MDDEPDIR)/LOpcodesGenerated.h.stub: /worker/build/js/src/jit/GenerateOpcodeFiles.py $(TOPSRCDIR)/js/src/jit/LIR.h $(TOPSRCDIR)/js/src/jit/shared/LIR-shared.h $(TOPSRCDIR)/js/src/jit/x86-shared/LIR-x86-shared.h $(TOPSRCDIR)/js/src/jit/x64/LIR-x64.h
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/js/src/jit/GenerateOpcodeFiles.py generate_lir_header $(TOPOBJDIR)/js/src/jit/LOpcodesGenerated.h $(TOPOBJDIR)/js/src/jit/$(MDDEPDIR)/LOpcodesGenerated.h.pp $(TOPOBJDIR)/js/src/jit/$(MDDEPDIR)/LOpcodesGenerated.h.stub $(TOPSRCDIR)/js/src/jit/LIR.h $(TOPSRCDIR)/js/src/jit/shared/LIR-shared.h $(TOPSRCDIR)/js/src/jit/x86-shared/LIR-x86-shared.h $(TOPSRCDIR)/js/src/jit/x64/LIR-x64.h)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/js/src/jit/$(MDDEPDIR)/CacheIROpsGenerated.h.stub
$(TOPOBJDIR)/js/src/jit/CacheIROpsGenerated.h: $(TOPOBJDIR)/js/src/jit/$(MDDEPDIR)/CacheIROpsGenerated.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/js/src/jit/$(MDDEPDIR)/CacheIROpsGenerated.h.pp
$(TOPOBJDIR)/js/src/jit/$(MDDEPDIR)/CacheIROpsGenerated.h.stub: /worker/build/js/src/jit/GenerateCacheIRFiles.py $(TOPSRCDIR)/js/src/jit/CacheIROps.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/js/src/jit/GenerateCacheIRFiles.py generate_cacheirops_header $(TOPOBJDIR)/js/src/jit/CacheIROpsGenerated.h $(TOPOBJDIR)/js/src/jit/$(MDDEPDIR)/CacheIROpsGenerated.h.pp $(TOPOBJDIR)/js/src/jit/$(MDDEPDIR)/CacheIROpsGenerated.h.stub $(TOPSRCDIR)/js/src/jit/CacheIROps.yaml)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/js/src/ctypes/libffi/$(MDDEPDIR)/fficonfig.h.stub
$(TOPOBJDIR)/js/src/ctypes/libffi/fficonfig.h: $(TOPOBJDIR)/js/src/ctypes/libffi/$(MDDEPDIR)/fficonfig.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/js/src/ctypes/libffi/$(MDDEPDIR)/fficonfig.h.pp
$(TOPOBJDIR)/js/src/ctypes/libffi/$(MDDEPDIR)/fficonfig.h.stub: /worker/build/python/mozbuild/mozbuild/action/process_define_files.py $(TOPSRCDIR)/js/src/ctypes/libffi/fficonfig.h.in
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/process_define_files.py process_define_file $(TOPOBJDIR)/js/src/ctypes/libffi/fficonfig.h $(TOPOBJDIR)/js/src/ctypes/libffi/$(MDDEPDIR)/fficonfig.h.pp $(TOPOBJDIR)/js/src/ctypes/libffi/$(MDDEPDIR)/fficonfig.h.stub $(TOPSRCDIR)/js/src/ctypes/libffi/fficonfig.h.in)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/js/src/ctypes/libffi/include/$(MDDEPDIR)/ffi.h.stub
$(TOPOBJDIR)/js/src/ctypes/libffi/include/ffi.h: $(TOPOBJDIR)/js/src/ctypes/libffi/include/$(MDDEPDIR)/ffi.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/js/src/ctypes/libffi/include/$(MDDEPDIR)/ffi.h.pp
$(TOPOBJDIR)/js/src/ctypes/libffi/include/$(MDDEPDIR)/ffi.h.stub: /worker/build/config/external/ffi/subst_header.py $(TOPSRCDIR)/js/src/ctypes/libffi/include/ffi.h.in
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/config/external/ffi/subst_header.py main $(TOPOBJDIR)/js/src/ctypes/libffi/include/ffi.h $(TOPOBJDIR)/js/src/ctypes/libffi/include/$(MDDEPDIR)/ffi.h.pp $(TOPOBJDIR)/js/src/ctypes/libffi/include/$(MDDEPDIR)/ffi.h.stub $(TOPSRCDIR)/js/src/ctypes/libffi/include/ffi.h.in)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/config/external/sqlite/$(MDDEPDIR)/libmozsqlite3.so.symbols.stub
$(TOPOBJDIR)/config/external/sqlite/libmozsqlite3.so.symbols: $(TOPOBJDIR)/config/external/sqlite/$(MDDEPDIR)/libmozsqlite3.so.symbols.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/config/external/sqlite/$(MDDEPDIR)/libmozsqlite3.so.symbols.pp
$(TOPOBJDIR)/config/external/sqlite/$(MDDEPDIR)/libmozsqlite3.so.symbols.stub: /worker/build/python/mozbuild/mozbuild/action/generate_symbols_file.py $(TOPSRCDIR)/third_party/sqlite3/src/sqlite.symbols
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/generate_symbols_file.py generate_symbols_file $(TOPOBJDIR)/config/external/sqlite/libmozsqlite3.so.symbols $(TOPOBJDIR)/config/external/sqlite/$(MDDEPDIR)/libmozsqlite3.so.symbols.pp $(TOPOBJDIR)/config/external/sqlite/$(MDDEPDIR)/libmozsqlite3.so.symbols.stub $(TOPSRCDIR)/third_party/sqlite3/src/sqlite.symbols )
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_cdef_apply_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/16bd_cdef_apply_tmpl.c: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_cdef_apply_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_cdef_apply_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_cdef_apply_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/cdef_apply_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/16bd_cdef_apply_tmpl.c $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_cdef_apply_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_cdef_apply_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/cdef_apply_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_cdef_apply_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/8bd_cdef_apply_tmpl.c: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_cdef_apply_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_cdef_apply_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_cdef_apply_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/cdef_apply_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/8bd_cdef_apply_tmpl.c $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_cdef_apply_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_cdef_apply_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/cdef_apply_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_cdef_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/16bd_cdef_tmpl.c: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_cdef_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_cdef_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_cdef_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/cdef_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/16bd_cdef_tmpl.c $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_cdef_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_cdef_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/cdef_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_cdef_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/8bd_cdef_tmpl.c: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_cdef_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_cdef_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_cdef_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/cdef_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/8bd_cdef_tmpl.c $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_cdef_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_cdef_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/cdef_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_fg_apply_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/16bd_fg_apply_tmpl.c: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_fg_apply_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_fg_apply_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_fg_apply_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/fg_apply_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/16bd_fg_apply_tmpl.c $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_fg_apply_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_fg_apply_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/fg_apply_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_fg_apply_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/8bd_fg_apply_tmpl.c: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_fg_apply_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_fg_apply_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_fg_apply_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/fg_apply_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/8bd_fg_apply_tmpl.c $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_fg_apply_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_fg_apply_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/fg_apply_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_film_grain_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/16bd_film_grain_tmpl.c: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_film_grain_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_film_grain_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_film_grain_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/film_grain_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/16bd_film_grain_tmpl.c $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_film_grain_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_film_grain_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/film_grain_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_film_grain_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/8bd_film_grain_tmpl.c: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_film_grain_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_film_grain_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_film_grain_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/film_grain_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/8bd_film_grain_tmpl.c $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_film_grain_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_film_grain_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/film_grain_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_ipred_prepare_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/16bd_ipred_prepare_tmpl.c: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_ipred_prepare_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_ipred_prepare_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_ipred_prepare_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/ipred_prepare_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/16bd_ipred_prepare_tmpl.c $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_ipred_prepare_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_ipred_prepare_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/ipred_prepare_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_ipred_prepare_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/8bd_ipred_prepare_tmpl.c: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_ipred_prepare_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_ipred_prepare_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_ipred_prepare_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/ipred_prepare_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/8bd_ipred_prepare_tmpl.c $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_ipred_prepare_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_ipred_prepare_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/ipred_prepare_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_ipred_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/16bd_ipred_tmpl.c: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_ipred_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_ipred_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_ipred_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/ipred_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/16bd_ipred_tmpl.c $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_ipred_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_ipred_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/ipred_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_ipred_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/8bd_ipred_tmpl.c: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_ipred_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_ipred_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_ipred_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/ipred_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/8bd_ipred_tmpl.c $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_ipred_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_ipred_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/ipred_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_itx_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/16bd_itx_tmpl.c: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_itx_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_itx_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_itx_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/itx_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/16bd_itx_tmpl.c $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_itx_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_itx_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/itx_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_itx_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/8bd_itx_tmpl.c: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_itx_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_itx_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_itx_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/itx_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/8bd_itx_tmpl.c $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_itx_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_itx_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/itx_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_lf_apply_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/16bd_lf_apply_tmpl.c: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_lf_apply_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_lf_apply_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_lf_apply_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/lf_apply_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/16bd_lf_apply_tmpl.c $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_lf_apply_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_lf_apply_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/lf_apply_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_lf_apply_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/8bd_lf_apply_tmpl.c: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_lf_apply_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_lf_apply_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_lf_apply_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/lf_apply_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/8bd_lf_apply_tmpl.c $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_lf_apply_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_lf_apply_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/lf_apply_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_loopfilter_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/16bd_loopfilter_tmpl.c: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_loopfilter_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_loopfilter_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_loopfilter_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/loopfilter_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/16bd_loopfilter_tmpl.c $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_loopfilter_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_loopfilter_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/loopfilter_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_loopfilter_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/8bd_loopfilter_tmpl.c: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_loopfilter_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_loopfilter_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_loopfilter_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/loopfilter_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/8bd_loopfilter_tmpl.c $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_loopfilter_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_loopfilter_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/loopfilter_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_looprestoration_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/16bd_looprestoration_tmpl.c: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_looprestoration_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_looprestoration_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_looprestoration_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/looprestoration_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/16bd_looprestoration_tmpl.c $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_looprestoration_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_looprestoration_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/looprestoration_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_looprestoration_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/8bd_looprestoration_tmpl.c: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_looprestoration_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_looprestoration_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_looprestoration_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/looprestoration_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/8bd_looprestoration_tmpl.c $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_looprestoration_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_looprestoration_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/looprestoration_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_lr_apply_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/16bd_lr_apply_tmpl.c: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_lr_apply_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_lr_apply_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_lr_apply_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/lr_apply_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/16bd_lr_apply_tmpl.c $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_lr_apply_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_lr_apply_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/lr_apply_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_lr_apply_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/8bd_lr_apply_tmpl.c: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_lr_apply_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_lr_apply_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_lr_apply_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/lr_apply_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/8bd_lr_apply_tmpl.c $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_lr_apply_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_lr_apply_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/lr_apply_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_mc_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/16bd_mc_tmpl.c: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_mc_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_mc_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_mc_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/mc_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/16bd_mc_tmpl.c $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_mc_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_mc_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/mc_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_mc_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/8bd_mc_tmpl.c: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_mc_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_mc_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_mc_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/mc_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/8bd_mc_tmpl.c $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_mc_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_mc_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/mc_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_recon_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/16bd_recon_tmpl.c: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_recon_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_recon_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_recon_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/recon_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/16bd_recon_tmpl.c $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_recon_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/16bd_recon_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/recon_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_recon_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/8bd_recon_tmpl.c: $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_recon_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_recon_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_recon_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/recon_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/8bd_recon_tmpl.c $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_recon_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/$(MDDEPDIR)/8bd_recon_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/recon_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_cdef_init_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/asm/16bd_cdef_init_tmpl.c: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_cdef_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_cdef_init_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_cdef_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/x86/cdef_init_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/asm/16bd_cdef_init_tmpl.c $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_cdef_init_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_cdef_init_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/x86/cdef_init_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_cdef_init_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/asm/8bd_cdef_init_tmpl.c: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_cdef_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_cdef_init_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_cdef_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/x86/cdef_init_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/asm/8bd_cdef_init_tmpl.c $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_cdef_init_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_cdef_init_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/x86/cdef_init_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_film_grain_init_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/asm/16bd_film_grain_init_tmpl.c: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_film_grain_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_film_grain_init_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_film_grain_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/x86/film_grain_init_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/asm/16bd_film_grain_init_tmpl.c $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_film_grain_init_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_film_grain_init_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/x86/film_grain_init_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_film_grain_init_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/asm/8bd_film_grain_init_tmpl.c: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_film_grain_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_film_grain_init_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_film_grain_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/x86/film_grain_init_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/asm/8bd_film_grain_init_tmpl.c $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_film_grain_init_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_film_grain_init_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/x86/film_grain_init_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_ipred_init_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/asm/16bd_ipred_init_tmpl.c: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_ipred_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_ipred_init_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_ipred_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/x86/ipred_init_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/asm/16bd_ipred_init_tmpl.c $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_ipred_init_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_ipred_init_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/x86/ipred_init_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_ipred_init_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/asm/8bd_ipred_init_tmpl.c: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_ipred_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_ipred_init_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_ipred_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/x86/ipred_init_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/asm/8bd_ipred_init_tmpl.c $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_ipred_init_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_ipred_init_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/x86/ipred_init_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_itx_init_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/asm/16bd_itx_init_tmpl.c: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_itx_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_itx_init_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_itx_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/x86/itx_init_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/asm/16bd_itx_init_tmpl.c $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_itx_init_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_itx_init_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/x86/itx_init_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_itx_init_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/asm/8bd_itx_init_tmpl.c: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_itx_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_itx_init_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_itx_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/x86/itx_init_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/asm/8bd_itx_init_tmpl.c $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_itx_init_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_itx_init_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/x86/itx_init_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_loopfilter_init_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/asm/16bd_loopfilter_init_tmpl.c: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_loopfilter_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_loopfilter_init_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_loopfilter_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/x86/loopfilter_init_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/asm/16bd_loopfilter_init_tmpl.c $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_loopfilter_init_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_loopfilter_init_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/x86/loopfilter_init_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_loopfilter_init_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/asm/8bd_loopfilter_init_tmpl.c: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_loopfilter_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_loopfilter_init_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_loopfilter_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/x86/loopfilter_init_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/asm/8bd_loopfilter_init_tmpl.c $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_loopfilter_init_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_loopfilter_init_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/x86/loopfilter_init_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_looprestoration_init_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/asm/16bd_looprestoration_init_tmpl.c: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_looprestoration_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_looprestoration_init_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_looprestoration_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/x86/looprestoration_init_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/asm/16bd_looprestoration_init_tmpl.c $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_looprestoration_init_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_looprestoration_init_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/x86/looprestoration_init_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_looprestoration_init_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/asm/8bd_looprestoration_init_tmpl.c: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_looprestoration_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_looprestoration_init_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_looprestoration_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/x86/looprestoration_init_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/asm/8bd_looprestoration_init_tmpl.c $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_looprestoration_init_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_looprestoration_init_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/x86/looprestoration_init_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_mc_init_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/asm/16bd_mc_init_tmpl.c: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_mc_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_mc_init_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_mc_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/x86/mc_init_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/asm/16bd_mc_init_tmpl.c $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_mc_init_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/16bd_mc_init_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/x86/mc_init_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_mc_init_tmpl.c.stub
$(TOPOBJDIR)/media/libdav1d/asm/8bd_mc_init_tmpl.c: $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_mc_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_mc_init_tmpl.c.pp
$(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_mc_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(TOPSRCDIR)/third_party/dav1d/src/x86/mc_init_tmpl.c
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define $(TOPOBJDIR)/media/libdav1d/asm/8bd_mc_init_tmpl.c $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_mc_init_tmpl.c.pp $(TOPOBJDIR)/media/libdav1d/asm/$(MDDEPDIR)/8bd_mc_init_tmpl.c.stub $(TOPSRCDIR)/third_party/dav1d/src/x86/mc_init_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/ffvpx/libavutil/$(MDDEPDIR)/libmozavutil.so.symbols.stub
$(TOPOBJDIR)/media/ffvpx/libavutil/libmozavutil.so.symbols: $(TOPOBJDIR)/media/ffvpx/libavutil/$(MDDEPDIR)/libmozavutil.so.symbols.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/ffvpx/libavutil/$(MDDEPDIR)/libmozavutil.so.symbols.pp
$(TOPOBJDIR)/media/ffvpx/libavutil/$(MDDEPDIR)/libmozavutil.so.symbols.stub: /worker/build/python/mozbuild/mozbuild/action/generate_symbols_file.py $(TOPSRCDIR)/media/ffvpx/libavutil/avutil.symbols
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/generate_symbols_file.py generate_symbols_file $(TOPOBJDIR)/media/ffvpx/libavutil/libmozavutil.so.symbols $(TOPOBJDIR)/media/ffvpx/libavutil/$(MDDEPDIR)/libmozavutil.so.symbols.pp $(TOPOBJDIR)/media/ffvpx/libavutil/$(MDDEPDIR)/libmozavutil.so.symbols.stub $(TOPSRCDIR)/media/ffvpx/libavutil/avutil.symbols )
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/ffvpx/libavcodec/$(MDDEPDIR)/libmozavcodec.so.symbols.stub
$(TOPOBJDIR)/media/ffvpx/libavcodec/libmozavcodec.so.symbols: $(TOPOBJDIR)/media/ffvpx/libavcodec/$(MDDEPDIR)/libmozavcodec.so.symbols.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/ffvpx/libavcodec/$(MDDEPDIR)/libmozavcodec.so.symbols.pp
$(TOPOBJDIR)/media/ffvpx/libavcodec/$(MDDEPDIR)/libmozavcodec.so.symbols.stub: /worker/build/python/mozbuild/mozbuild/action/generate_symbols_file.py $(TOPSRCDIR)/media/ffvpx/libavcodec/avcodec.symbols
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/generate_symbols_file.py generate_symbols_file $(TOPOBJDIR)/media/ffvpx/libavcodec/libmozavcodec.so.symbols $(TOPOBJDIR)/media/ffvpx/libavcodec/$(MDDEPDIR)/libmozavcodec.so.symbols.pp $(TOPOBJDIR)/media/ffvpx/libavcodec/$(MDDEPDIR)/libmozavcodec.so.symbols.stub $(TOPSRCDIR)/media/ffvpx/libavcodec/avcodec.symbols )
	@$(TOUCH) $@

default: $(TOPOBJDIR)/media/mp4parse-rust/$(MDDEPDIR)/mp4parse_ffi_generated.h.stub
$(TOPOBJDIR)/media/mp4parse-rust/mp4parse_ffi_generated.h: $(TOPOBJDIR)/media/mp4parse-rust/$(MDDEPDIR)/mp4parse_ffi_generated.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/media/mp4parse-rust/$(MDDEPDIR)/mp4parse_ffi_generated.h.pp
$(TOPOBJDIR)/media/mp4parse-rust/$(MDDEPDIR)/mp4parse_ffi_generated.h.stub: /worker/build/build/RunCbindgen.py $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/third_party/rust/mp4parse_capi
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate $(TOPOBJDIR)/media/mp4parse-rust/mp4parse_ffi_generated.h $(TOPOBJDIR)/media/mp4parse-rust/$(MDDEPDIR)/mp4parse_ffi_generated.h.pp $(TOPOBJDIR)/media/mp4parse-rust/$(MDDEPDIR)/mp4parse_ffi_generated.h.stub $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/third_party/rust/mp4parse_capi)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/valid-sth.inc.stub
$(TOPOBJDIR)/security/ct/tests/gtest/valid-sth.inc: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/valid-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/valid-sth.inc.pp
$(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/valid-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(TOPSRCDIR)/security/ct/tests/gtest/valid-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main $(TOPOBJDIR)/security/ct/tests/gtest/valid-sth.inc $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/valid-sth.inc.pp $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/valid-sth.inc.stub $(TOPSRCDIR)/security/ct/tests/gtest/valid-sth.inc.tbs)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/valid-with-extension-sth.inc.stub
$(TOPOBJDIR)/security/ct/tests/gtest/valid-with-extension-sth.inc: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/valid-with-extension-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/valid-with-extension-sth.inc.pp
$(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/valid-with-extension-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(TOPSRCDIR)/security/ct/tests/gtest/valid-with-extension-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main $(TOPOBJDIR)/security/ct/tests/gtest/valid-with-extension-sth.inc $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/valid-with-extension-sth.inc.pp $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/valid-with-extension-sth.inc.stub $(TOPSRCDIR)/security/ct/tests/gtest/valid-with-extension-sth.inc.tbs)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/valid-secp521r1-sha512-sth.inc.stub
$(TOPOBJDIR)/security/ct/tests/gtest/valid-secp521r1-sha512-sth.inc: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/valid-secp521r1-sha512-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/valid-secp521r1-sha512-sth.inc.pp
$(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/valid-secp521r1-sha512-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(TOPSRCDIR)/security/ct/tests/gtest/valid-secp521r1-sha512-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main $(TOPOBJDIR)/security/ct/tests/gtest/valid-secp521r1-sha512-sth.inc $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/valid-secp521r1-sha512-sth.inc.pp $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/valid-secp521r1-sha512-sth.inc.stub $(TOPSRCDIR)/security/ct/tests/gtest/valid-secp521r1-sha512-sth.inc.tbs)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/signature-covers-log-id-sth.inc.stub
$(TOPOBJDIR)/security/ct/tests/gtest/signature-covers-log-id-sth.inc: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/signature-covers-log-id-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/signature-covers-log-id-sth.inc.pp
$(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/signature-covers-log-id-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(TOPSRCDIR)/security/ct/tests/gtest/signature-covers-log-id-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main $(TOPOBJDIR)/security/ct/tests/gtest/signature-covers-log-id-sth.inc $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/signature-covers-log-id-sth.inc.pp $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/signature-covers-log-id-sth.inc.stub $(TOPSRCDIR)/security/ct/tests/gtest/signature-covers-log-id-sth.inc.tbs)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/wrong-spki-sth.inc.stub
$(TOPOBJDIR)/security/ct/tests/gtest/wrong-spki-sth.inc: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/wrong-spki-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/wrong-spki-sth.inc.pp
$(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/wrong-spki-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(TOPSRCDIR)/security/ct/tests/gtest/wrong-spki-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main $(TOPOBJDIR)/security/ct/tests/gtest/wrong-spki-sth.inc $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/wrong-spki-sth.inc.pp $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/wrong-spki-sth.inc.stub $(TOPSRCDIR)/security/ct/tests/gtest/wrong-spki-sth.inc.tbs)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/wrong-signing-key-sth.inc.stub
$(TOPOBJDIR)/security/ct/tests/gtest/wrong-signing-key-sth.inc: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/wrong-signing-key-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/wrong-signing-key-sth.inc.pp
$(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/wrong-signing-key-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(TOPSRCDIR)/security/ct/tests/gtest/wrong-signing-key-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main $(TOPOBJDIR)/security/ct/tests/gtest/wrong-signing-key-sth.inc $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/wrong-signing-key-sth.inc.pp $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/wrong-signing-key-sth.inc.stub $(TOPSRCDIR)/security/ct/tests/gtest/wrong-signing-key-sth.inc.tbs)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-log-id-sth.inc.stub
$(TOPOBJDIR)/security/ct/tests/gtest/missing-log-id-sth.inc: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-log-id-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-log-id-sth.inc.pp
$(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-log-id-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(TOPSRCDIR)/security/ct/tests/gtest/missing-log-id-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main $(TOPOBJDIR)/security/ct/tests/gtest/missing-log-id-sth.inc $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-log-id-sth.inc.pp $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-log-id-sth.inc.stub $(TOPSRCDIR)/security/ct/tests/gtest/missing-log-id-sth.inc.tbs)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-timestamp-sth.inc.stub
$(TOPOBJDIR)/security/ct/tests/gtest/missing-timestamp-sth.inc: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-timestamp-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-timestamp-sth.inc.pp
$(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-timestamp-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(TOPSRCDIR)/security/ct/tests/gtest/missing-timestamp-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main $(TOPOBJDIR)/security/ct/tests/gtest/missing-timestamp-sth.inc $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-timestamp-sth.inc.pp $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-timestamp-sth.inc.stub $(TOPSRCDIR)/security/ct/tests/gtest/missing-timestamp-sth.inc.tbs)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-tree-size-sth.inc.stub
$(TOPOBJDIR)/security/ct/tests/gtest/missing-tree-size-sth.inc: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-tree-size-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-tree-size-sth.inc.pp
$(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-tree-size-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(TOPSRCDIR)/security/ct/tests/gtest/missing-tree-size-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main $(TOPOBJDIR)/security/ct/tests/gtest/missing-tree-size-sth.inc $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-tree-size-sth.inc.pp $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-tree-size-sth.inc.stub $(TOPSRCDIR)/security/ct/tests/gtest/missing-tree-size-sth.inc.tbs)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-root-hash-sth.inc.stub
$(TOPOBJDIR)/security/ct/tests/gtest/missing-root-hash-sth.inc: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-root-hash-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-root-hash-sth.inc.pp
$(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-root-hash-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(TOPSRCDIR)/security/ct/tests/gtest/missing-root-hash-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main $(TOPOBJDIR)/security/ct/tests/gtest/missing-root-hash-sth.inc $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-root-hash-sth.inc.pp $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-root-hash-sth.inc.stub $(TOPSRCDIR)/security/ct/tests/gtest/missing-root-hash-sth.inc.tbs)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-extensions-sth.inc.stub
$(TOPOBJDIR)/security/ct/tests/gtest/missing-extensions-sth.inc: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-extensions-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-extensions-sth.inc.pp
$(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-extensions-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(TOPSRCDIR)/security/ct/tests/gtest/missing-extensions-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main $(TOPOBJDIR)/security/ct/tests/gtest/missing-extensions-sth.inc $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-extensions-sth.inc.pp $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/missing-extensions-sth.inc.stub $(TOPSRCDIR)/security/ct/tests/gtest/missing-extensions-sth.inc.tbs)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-log-id-sth.inc.stub
$(TOPOBJDIR)/security/ct/tests/gtest/truncated-log-id-sth.inc: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-log-id-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-log-id-sth.inc.pp
$(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-log-id-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(TOPSRCDIR)/security/ct/tests/gtest/truncated-log-id-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main $(TOPOBJDIR)/security/ct/tests/gtest/truncated-log-id-sth.inc $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-log-id-sth.inc.pp $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-log-id-sth.inc.stub $(TOPSRCDIR)/security/ct/tests/gtest/truncated-log-id-sth.inc.tbs)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-timestamp-sth.inc.stub
$(TOPOBJDIR)/security/ct/tests/gtest/truncated-timestamp-sth.inc: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-timestamp-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-timestamp-sth.inc.pp
$(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-timestamp-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(TOPSRCDIR)/security/ct/tests/gtest/truncated-timestamp-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main $(TOPOBJDIR)/security/ct/tests/gtest/truncated-timestamp-sth.inc $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-timestamp-sth.inc.pp $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-timestamp-sth.inc.stub $(TOPSRCDIR)/security/ct/tests/gtest/truncated-timestamp-sth.inc.tbs)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-tree-size-sth.inc.stub
$(TOPOBJDIR)/security/ct/tests/gtest/truncated-tree-size-sth.inc: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-tree-size-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-tree-size-sth.inc.pp
$(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-tree-size-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(TOPSRCDIR)/security/ct/tests/gtest/truncated-tree-size-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main $(TOPOBJDIR)/security/ct/tests/gtest/truncated-tree-size-sth.inc $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-tree-size-sth.inc.pp $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-tree-size-sth.inc.stub $(TOPSRCDIR)/security/ct/tests/gtest/truncated-tree-size-sth.inc.tbs)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-root-hash-sth.inc.stub
$(TOPOBJDIR)/security/ct/tests/gtest/truncated-root-hash-sth.inc: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-root-hash-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-root-hash-sth.inc.pp
$(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-root-hash-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(TOPSRCDIR)/security/ct/tests/gtest/truncated-root-hash-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main $(TOPOBJDIR)/security/ct/tests/gtest/truncated-root-hash-sth.inc $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-root-hash-sth.inc.pp $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-root-hash-sth.inc.stub $(TOPSRCDIR)/security/ct/tests/gtest/truncated-root-hash-sth.inc.tbs)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-extension-sth.inc.stub
$(TOPOBJDIR)/security/ct/tests/gtest/truncated-extension-sth.inc: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-extension-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-extension-sth.inc.pp
$(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-extension-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(TOPSRCDIR)/security/ct/tests/gtest/truncated-extension-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main $(TOPOBJDIR)/security/ct/tests/gtest/truncated-extension-sth.inc $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-extension-sth.inc.pp $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/truncated-extension-sth.inc.stub $(TOPSRCDIR)/security/ct/tests/gtest/truncated-extension-sth.inc.tbs)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/rsa-signer-rsa-spki-sth.inc.stub
$(TOPOBJDIR)/security/ct/tests/gtest/rsa-signer-rsa-spki-sth.inc: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/rsa-signer-rsa-spki-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/rsa-signer-rsa-spki-sth.inc.pp
$(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/rsa-signer-rsa-spki-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(TOPSRCDIR)/security/ct/tests/gtest/rsa-signer-rsa-spki-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main $(TOPOBJDIR)/security/ct/tests/gtest/rsa-signer-rsa-spki-sth.inc $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/rsa-signer-rsa-spki-sth.inc.pp $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/rsa-signer-rsa-spki-sth.inc.stub $(TOPSRCDIR)/security/ct/tests/gtest/rsa-signer-rsa-spki-sth.inc.tbs)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/rsa-signer-ec-spki-sth.inc.stub
$(TOPOBJDIR)/security/ct/tests/gtest/rsa-signer-ec-spki-sth.inc: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/rsa-signer-ec-spki-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/rsa-signer-ec-spki-sth.inc.pp
$(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/rsa-signer-ec-spki-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(TOPSRCDIR)/security/ct/tests/gtest/rsa-signer-ec-spki-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main $(TOPOBJDIR)/security/ct/tests/gtest/rsa-signer-ec-spki-sth.inc $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/rsa-signer-ec-spki-sth.inc.pp $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/rsa-signer-ec-spki-sth.inc.stub $(TOPSRCDIR)/security/ct/tests/gtest/rsa-signer-ec-spki-sth.inc.tbs)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/ec-signer-rsa-spki-sth.inc.stub
$(TOPOBJDIR)/security/ct/tests/gtest/ec-signer-rsa-spki-sth.inc: $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/ec-signer-rsa-spki-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/ec-signer-rsa-spki-sth.inc.pp
$(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/ec-signer-rsa-spki-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(TOPSRCDIR)/security/ct/tests/gtest/ec-signer-rsa-spki-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main $(TOPOBJDIR)/security/ct/tests/gtest/ec-signer-rsa-spki-sth.inc $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/ec-signer-rsa-spki-sth.inc.pp $(TOPOBJDIR)/security/ct/tests/gtest/$(MDDEPDIR)/ec-signer-rsa-spki-sth.inc.stub $(TOPSRCDIR)/security/ct/tests/gtest/ec-signer-rsa-spki-sth.inc.tbs)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/apps/$(MDDEPDIR)/xpcshell.inc.stub
$(TOPOBJDIR)/security/apps/xpcshell.inc: $(TOPOBJDIR)/security/apps/$(MDDEPDIR)/xpcshell.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/apps/$(MDDEPDIR)/xpcshell.inc.pp
$(TOPOBJDIR)/security/apps/$(MDDEPDIR)/xpcshell.inc.stub: /worker/build/security/apps/gen_cert_header.py $(TOPSRCDIR)/security/manager/ssl/tests/unit/test_signed_apps/xpcshellTestRoot.der
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/apps/gen_cert_header.py xpcshellRoot $(TOPOBJDIR)/security/apps/xpcshell.inc $(TOPOBJDIR)/security/apps/$(MDDEPDIR)/xpcshell.inc.pp $(TOPOBJDIR)/security/apps/$(MDDEPDIR)/xpcshell.inc.stub $(TOPSRCDIR)/security/manager/ssl/tests/unit/test_signed_apps/xpcshellTestRoot.der)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/apps/$(MDDEPDIR)/addons-public.inc.stub
$(TOPOBJDIR)/security/apps/addons-public.inc: $(TOPOBJDIR)/security/apps/$(MDDEPDIR)/addons-public.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/apps/$(MDDEPDIR)/addons-public.inc.pp
$(TOPOBJDIR)/security/apps/$(MDDEPDIR)/addons-public.inc.stub: /worker/build/security/apps/gen_cert_header.py $(TOPSRCDIR)/security/apps/addons-public.crt
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/apps/gen_cert_header.py addonsPublicRoot $(TOPOBJDIR)/security/apps/addons-public.inc $(TOPOBJDIR)/security/apps/$(MDDEPDIR)/addons-public.inc.pp $(TOPOBJDIR)/security/apps/$(MDDEPDIR)/addons-public.inc.stub $(TOPSRCDIR)/security/apps/addons-public.crt)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/apps/$(MDDEPDIR)/addons-public-intermediate.inc.stub
$(TOPOBJDIR)/security/apps/addons-public-intermediate.inc: $(TOPOBJDIR)/security/apps/$(MDDEPDIR)/addons-public-intermediate.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/apps/$(MDDEPDIR)/addons-public-intermediate.inc.pp
$(TOPOBJDIR)/security/apps/$(MDDEPDIR)/addons-public-intermediate.inc.stub: /worker/build/security/apps/gen_cert_header.py $(TOPSRCDIR)/security/apps/addons-public-intermediate.crt
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/apps/gen_cert_header.py addonsPublicIntermediate $(TOPOBJDIR)/security/apps/addons-public-intermediate.inc $(TOPOBJDIR)/security/apps/$(MDDEPDIR)/addons-public-intermediate.inc.pp $(TOPOBJDIR)/security/apps/$(MDDEPDIR)/addons-public-intermediate.inc.stub $(TOPSRCDIR)/security/apps/addons-public-intermediate.crt)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/apps/$(MDDEPDIR)/addons-stage.inc.stub
$(TOPOBJDIR)/security/apps/addons-stage.inc: $(TOPOBJDIR)/security/apps/$(MDDEPDIR)/addons-stage.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/apps/$(MDDEPDIR)/addons-stage.inc.pp
$(TOPOBJDIR)/security/apps/$(MDDEPDIR)/addons-stage.inc.stub: /worker/build/security/apps/gen_cert_header.py $(TOPSRCDIR)/security/apps/addons-stage.crt
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/apps/gen_cert_header.py addonsStageRoot $(TOPOBJDIR)/security/apps/addons-stage.inc $(TOPOBJDIR)/security/apps/$(MDDEPDIR)/addons-stage.inc.pp $(TOPOBJDIR)/security/apps/$(MDDEPDIR)/addons-stage.inc.stub $(TOPSRCDIR)/security/apps/addons-stage.crt)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/xpcom/$(MDDEPDIR)/xpcom-config.h.stub
$(TOPOBJDIR)/xpcom/xpcom-config.h: $(TOPOBJDIR)/xpcom/$(MDDEPDIR)/xpcom-config.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/xpcom/$(MDDEPDIR)/xpcom-config.h.pp
$(TOPOBJDIR)/xpcom/$(MDDEPDIR)/xpcom-config.h.stub: /worker/build/python/mozbuild/mozbuild/action/process_define_files.py $(TOPSRCDIR)/xpcom/xpcom-config.h.in
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/process_define_files.py process_define_file $(TOPOBJDIR)/xpcom/xpcom-config.h $(TOPOBJDIR)/xpcom/$(MDDEPDIR)/xpcom-config.h.pp $(TOPOBJDIR)/xpcom/$(MDDEPDIR)/xpcom-config.h.stub $(TOPSRCDIR)/xpcom/xpcom-config.h.in)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/xpcom/idl-parser/xpidl/$(MDDEPDIR)/xpidl.stub.stub
$(TOPOBJDIR)/xpcom/idl-parser/xpidl/xpidl.stub: $(TOPOBJDIR)/xpcom/idl-parser/xpidl/$(MDDEPDIR)/xpidl.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/xpcom/idl-parser/xpidl/$(MDDEPDIR)/xpidl.stub.pp
$(TOPOBJDIR)/xpcom/idl-parser/xpidl/$(MDDEPDIR)/xpidl.stub.stub: /worker/build/xpcom/idl-parser/xpidl/header.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/xpcom/idl-parser/xpidl/header.py main $(TOPOBJDIR)/xpcom/idl-parser/xpidl/xpidl.stub $(TOPOBJDIR)/xpcom/idl-parser/xpidl/$(MDDEPDIR)/xpidl.stub.pp $(TOPOBJDIR)/xpcom/idl-parser/xpidl/$(MDDEPDIR)/xpidl.stub.stub)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/xpcom/base/$(MDDEPDIR)/ErrorList.h.stub
$(TOPOBJDIR)/xpcom/base/ErrorList.h: $(TOPOBJDIR)/xpcom/base/$(MDDEPDIR)/ErrorList.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/xpcom/base/$(MDDEPDIR)/ErrorList.h.pp
$(TOPOBJDIR)/xpcom/base/$(MDDEPDIR)/ErrorList.h.stub: /worker/build/xpcom/base/ErrorList.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/xpcom/base/ErrorList.py error_list_h $(TOPOBJDIR)/xpcom/base/ErrorList.h $(TOPOBJDIR)/xpcom/base/$(MDDEPDIR)/ErrorList.h.pp $(TOPOBJDIR)/xpcom/base/$(MDDEPDIR)/ErrorList.h.stub)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/xpcom/base/$(MDDEPDIR)/ErrorNamesInternal.h.stub
$(TOPOBJDIR)/xpcom/base/ErrorNamesInternal.h: $(TOPOBJDIR)/xpcom/base/$(MDDEPDIR)/ErrorNamesInternal.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/xpcom/base/$(MDDEPDIR)/ErrorNamesInternal.h.pp
$(TOPOBJDIR)/xpcom/base/$(MDDEPDIR)/ErrorNamesInternal.h.stub: /worker/build/xpcom/base/ErrorList.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/xpcom/base/ErrorList.py error_names_internal_h $(TOPOBJDIR)/xpcom/base/ErrorNamesInternal.h $(TOPOBJDIR)/xpcom/base/$(MDDEPDIR)/ErrorNamesInternal.h.pp $(TOPOBJDIR)/xpcom/base/$(MDDEPDIR)/ErrorNamesInternal.h.stub)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/xpcom/base/$(MDDEPDIR)/error_list.rs.stub
$(TOPOBJDIR)/xpcom/base/error_list.rs: $(TOPOBJDIR)/xpcom/base/$(MDDEPDIR)/error_list.rs.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/xpcom/base/$(MDDEPDIR)/error_list.rs.pp
$(TOPOBJDIR)/xpcom/base/$(MDDEPDIR)/error_list.rs.stub: /worker/build/xpcom/base/ErrorList.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/xpcom/base/ErrorList.py error_list_rs $(TOPOBJDIR)/xpcom/base/error_list.rs $(TOPOBJDIR)/xpcom/base/$(MDDEPDIR)/error_list.rs.pp $(TOPOBJDIR)/xpcom/base/$(MDDEPDIR)/error_list.rs.stub)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/xpcom/base/$(MDDEPDIR)/gk_rust_utils_ffi_generated.h.stub
$(TOPOBJDIR)/xpcom/base/gk_rust_utils_ffi_generated.h: $(TOPOBJDIR)/xpcom/base/$(MDDEPDIR)/gk_rust_utils_ffi_generated.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/xpcom/base/$(MDDEPDIR)/gk_rust_utils_ffi_generated.h.pp
$(TOPOBJDIR)/xpcom/base/$(MDDEPDIR)/gk_rust_utils_ffi_generated.h.stub: /worker/build/build/RunCbindgen.py $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/xpcom/rust/gkrust_utils
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate $(TOPOBJDIR)/xpcom/base/gk_rust_utils_ffi_generated.h $(TOPOBJDIR)/xpcom/base/$(MDDEPDIR)/gk_rust_utils_ffi_generated.h.pp $(TOPOBJDIR)/xpcom/base/$(MDDEPDIR)/gk_rust_utils_ffi_generated.h.stub $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/xpcom/rust/gkrust_utils)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/xpcom/ds/$(MDDEPDIR)/nsGkAtomList.h.stub
$(TOPOBJDIR)/xpcom/ds/nsGkAtomList.h: $(TOPOBJDIR)/xpcom/ds/$(MDDEPDIR)/nsGkAtomList.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/xpcom/ds/$(MDDEPDIR)/nsGkAtomList.h.pp
$(TOPOBJDIR)/xpcom/ds/$(MDDEPDIR)/nsGkAtomList.h.stub: /worker/build/xpcom/ds/StaticAtoms.py $(TOPSRCDIR)/xpcom/ds/Atom.py $(TOPSRCDIR)/xpcom/ds/HTMLAtoms.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/xpcom/ds/StaticAtoms.py generate_nsgkatomlist_h $(TOPOBJDIR)/xpcom/ds/nsGkAtomList.h $(TOPOBJDIR)/xpcom/ds/$(MDDEPDIR)/nsGkAtomList.h.pp $(TOPOBJDIR)/xpcom/ds/$(MDDEPDIR)/nsGkAtomList.h.stub $(TOPSRCDIR)/xpcom/ds/Atom.py $(TOPSRCDIR)/xpcom/ds/HTMLAtoms.py)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/xpcom/ds/$(MDDEPDIR)/nsGkAtomConsts.h.stub
$(TOPOBJDIR)/xpcom/ds/nsGkAtomConsts.h: $(TOPOBJDIR)/xpcom/ds/$(MDDEPDIR)/nsGkAtomConsts.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/xpcom/ds/$(MDDEPDIR)/nsGkAtomConsts.h.pp
$(TOPOBJDIR)/xpcom/ds/$(MDDEPDIR)/nsGkAtomConsts.h.stub: /worker/build/xpcom/ds/StaticAtoms.py $(TOPSRCDIR)/xpcom/ds/Atom.py $(TOPSRCDIR)/xpcom/ds/HTMLAtoms.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/xpcom/ds/StaticAtoms.py generate_nsgkatomconsts_h $(TOPOBJDIR)/xpcom/ds/nsGkAtomConsts.h $(TOPOBJDIR)/xpcom/ds/$(MDDEPDIR)/nsGkAtomConsts.h.pp $(TOPOBJDIR)/xpcom/ds/$(MDDEPDIR)/nsGkAtomConsts.h.stub $(TOPSRCDIR)/xpcom/ds/Atom.py $(TOPSRCDIR)/xpcom/ds/HTMLAtoms.py)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/xpcom/components/$(MDDEPDIR)/Components.h.stub
$(TOPOBJDIR)/xpcom/components/Components.h: $(TOPOBJDIR)/xpcom/components/$(MDDEPDIR)/Components.h.stub ;
$(TOPOBJDIR)/xpcom/components/StaticComponentData.h: $(TOPOBJDIR)/xpcom/components/$(MDDEPDIR)/Components.h.stub ;
$(TOPOBJDIR)/xpcom/components/StaticComponents.cpp: $(TOPOBJDIR)/xpcom/components/$(MDDEPDIR)/Components.h.stub ;
$(TOPOBJDIR)/xpcom/components/services.json: $(TOPOBJDIR)/xpcom/components/$(MDDEPDIR)/Components.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/xpcom/components/$(MDDEPDIR)/Components.h.pp
$(TOPOBJDIR)/xpcom/components/$(MDDEPDIR)/Components.h.stub: /worker/build/xpcom/components/gen_static_components.py $(TOPOBJDIR)/xpcom/components/manifest-lists.json $(TOPSRCDIR)/xpcom/components/StaticComponents.cpp.in
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/xpcom/components/gen_static_components.py main $(TOPOBJDIR)/xpcom/components/Components.h $(TOPOBJDIR)/xpcom/components/$(MDDEPDIR)/Components.h.pp $(TOPOBJDIR)/xpcom/components/$(MDDEPDIR)/Components.h.stub $(TOPOBJDIR)/xpcom/components/manifest-lists.json $(TOPSRCDIR)/xpcom/components/StaticComponents.cpp.in)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/xpcom/build/$(MDDEPDIR)/Services.cpp.stub
$(TOPOBJDIR)/xpcom/build/Services.cpp: $(TOPOBJDIR)/xpcom/build/$(MDDEPDIR)/Services.cpp.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/xpcom/build/$(MDDEPDIR)/Services.cpp.pp
$(TOPOBJDIR)/xpcom/build/$(MDDEPDIR)/Services.cpp.stub: /worker/build/xpcom/build/Services.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/xpcom/build/Services.py services_cpp $(TOPOBJDIR)/xpcom/build/Services.cpp $(TOPOBJDIR)/xpcom/build/$(MDDEPDIR)/Services.cpp.pp $(TOPOBJDIR)/xpcom/build/$(MDDEPDIR)/Services.cpp.stub)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/xpcom/build/$(MDDEPDIR)/Services.h.stub
$(TOPOBJDIR)/xpcom/build/Services.h: $(TOPOBJDIR)/xpcom/build/$(MDDEPDIR)/Services.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/xpcom/build/$(MDDEPDIR)/Services.h.pp
$(TOPOBJDIR)/xpcom/build/$(MDDEPDIR)/Services.h.stub: /worker/build/xpcom/build/Services.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/xpcom/build/Services.py services_h $(TOPOBJDIR)/xpcom/build/Services.h $(TOPOBJDIR)/xpcom/build/$(MDDEPDIR)/Services.h.pp $(TOPOBJDIR)/xpcom/build/$(MDDEPDIR)/Services.h.stub)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/xpcom/build/$(MDDEPDIR)/services.rs.stub
$(TOPOBJDIR)/xpcom/build/services.rs: $(TOPOBJDIR)/xpcom/build/$(MDDEPDIR)/services.rs.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/xpcom/build/$(MDDEPDIR)/services.rs.pp
$(TOPOBJDIR)/xpcom/build/$(MDDEPDIR)/services.rs.stub: /worker/build/xpcom/build/Services.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/xpcom/build/Services.py services_rs $(TOPOBJDIR)/xpcom/build/services.rs $(TOPOBJDIR)/xpcom/build/$(MDDEPDIR)/services.rs.pp $(TOPOBJDIR)/xpcom/build/$(MDDEPDIR)/services.rs.stub)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/xpcom/tests/gtest/$(MDDEPDIR)/dafsa_test_1.inc.stub
$(TOPOBJDIR)/xpcom/tests/gtest/dafsa_test_1.inc: $(TOPOBJDIR)/xpcom/tests/gtest/$(MDDEPDIR)/dafsa_test_1.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/xpcom/tests/gtest/$(MDDEPDIR)/dafsa_test_1.inc.pp
$(TOPOBJDIR)/xpcom/tests/gtest/$(MDDEPDIR)/dafsa_test_1.inc.stub: /worker/build/xpcom/ds/tools/make_dafsa.py $(TOPSRCDIR)/xpcom/tests/gtest/dafsa_test_1.dat
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/xpcom/ds/tools/make_dafsa.py main $(TOPOBJDIR)/xpcom/tests/gtest/dafsa_test_1.inc $(TOPOBJDIR)/xpcom/tests/gtest/$(MDDEPDIR)/dafsa_test_1.inc.pp $(TOPOBJDIR)/xpcom/tests/gtest/$(MDDEPDIR)/dafsa_test_1.inc.stub $(TOPSRCDIR)/xpcom/tests/gtest/dafsa_test_1.dat)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub
$(TOPOBJDIR)/modules/libpref/init/StaticPrefListAll.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefsAll.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_accessibility.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_alerts.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_apz.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_beacon.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_bidi.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_browser.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_canvas.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_channelclassifier.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_clipboard.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_content.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_converter.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_datareporting.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_device.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_devtools.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_docshell.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_dom.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_editor.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_extensions.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_findbar.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_fission.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_font.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_full_screen_api.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_general.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_geo.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_gfx.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_gl.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_html5.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_idle_period.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_image.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_intl.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_javascript.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_layers.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_layout.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_mathml.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_media.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_mousewheel.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_network.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_nglayout.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_page_load.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_permissions.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_plain_text.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_plugin.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_plugins.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_preferences.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_print.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_privacy.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_prompts.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_security.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_sidebar.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_slider.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_storage.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_svg.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_telemetry.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_test.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_threads.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_timer.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_toolkit.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_ui.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_urlclassifier.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_view_source.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_webgl.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_widget.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_xul.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefList_zoom.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_accessibility.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_alerts.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_apz.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_beacon.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_bidi.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_browser.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_canvas.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_channelclassifier.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_clipboard.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_content.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_converter.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_datareporting.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_device.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_devtools.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_docshell.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_dom.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_editor.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_extensions.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_findbar.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_fission.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_font.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_full_screen_api.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_general.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_geo.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_gfx.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_gl.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_html5.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_idle_period.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_image.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_intl.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_javascript.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_layers.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_layout.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_mathml.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_media.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_mousewheel.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_network.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_nglayout.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_page_load.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_permissions.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_plain_text.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_plugin.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_plugins.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_preferences.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_print.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_privacy.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_prompts.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_security.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_sidebar.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_slider.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_storage.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_svg.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_telemetry.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_test.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_threads.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_timer.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_toolkit.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_ui.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_urlclassifier.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_view_source.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_webgl.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_widget.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_xul.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/StaticPrefs_zoom.h: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/init/StaticPrefsCGetters.cpp: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
$(TOPOBJDIR)/modules/libpref/static_prefs.rs: $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.pp
$(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub: /worker/build/modules/libpref/init/generate_static_pref_list.py $(TOPSRCDIR)/modules/libpref/init/StaticPrefList.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/modules/libpref/init/generate_static_pref_list.py emit_code $(TOPOBJDIR)/modules/libpref/init/StaticPrefListAll.h $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.pp $(TOPOBJDIR)/modules/libpref/init/$(MDDEPDIR)/StaticPrefListAll.h.stub $(TOPSRCDIR)/modules/libpref/init/StaticPrefList.yaml)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/intl/hyphenation/glue/$(MDDEPDIR)/mapped_hyph.h.stub
$(TOPOBJDIR)/intl/hyphenation/glue/mapped_hyph.h: $(TOPOBJDIR)/intl/hyphenation/glue/$(MDDEPDIR)/mapped_hyph.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/intl/hyphenation/glue/$(MDDEPDIR)/mapped_hyph.h.pp
$(TOPOBJDIR)/intl/hyphenation/glue/$(MDDEPDIR)/mapped_hyph.h.stub: /worker/build/build/RunCbindgen.py $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/third_party/rust/mapped_hyph
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate $(TOPOBJDIR)/intl/hyphenation/glue/mapped_hyph.h $(TOPOBJDIR)/intl/hyphenation/glue/$(MDDEPDIR)/mapped_hyph.h.pp $(TOPOBJDIR)/intl/hyphenation/glue/$(MDDEPDIR)/mapped_hyph.h.stub $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/third_party/rust/mapped_hyph)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/intl/locale/$(MDDEPDIR)/encodingsgroups.properties.h.stub
$(TOPOBJDIR)/intl/locale/encodingsgroups.properties.h: $(TOPOBJDIR)/intl/locale/$(MDDEPDIR)/encodingsgroups.properties.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/intl/locale/$(MDDEPDIR)/encodingsgroups.properties.h.pp
$(TOPOBJDIR)/intl/locale/$(MDDEPDIR)/encodingsgroups.properties.h.stub: /worker/build/intl/locale/props2arrays.py $(TOPSRCDIR)/intl/locale/encodingsgroups.properties
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/intl/locale/props2arrays.py main $(TOPOBJDIR)/intl/locale/encodingsgroups.properties.h $(TOPOBJDIR)/intl/locale/$(MDDEPDIR)/encodingsgroups.properties.h.pp $(TOPOBJDIR)/intl/locale/$(MDDEPDIR)/encodingsgroups.properties.h.stub $(TOPSRCDIR)/intl/locale/encodingsgroups.properties)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/intl/locale/$(MDDEPDIR)/fluent_langneg_ffi_generated.h.stub
$(TOPOBJDIR)/intl/locale/fluent_langneg_ffi_generated.h: $(TOPOBJDIR)/intl/locale/$(MDDEPDIR)/fluent_langneg_ffi_generated.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/intl/locale/$(MDDEPDIR)/fluent_langneg_ffi_generated.h.pp
$(TOPOBJDIR)/intl/locale/$(MDDEPDIR)/fluent_langneg_ffi_generated.h.stub: /worker/build/build/RunCbindgen.py $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/intl/locale/rust/fluent-langneg-ffi
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate $(TOPOBJDIR)/intl/locale/fluent_langneg_ffi_generated.h $(TOPOBJDIR)/intl/locale/$(MDDEPDIR)/fluent_langneg_ffi_generated.h.pp $(TOPOBJDIR)/intl/locale/$(MDDEPDIR)/fluent_langneg_ffi_generated.h.stub $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/intl/locale/rust/fluent-langneg-ffi)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/intl/locale/$(MDDEPDIR)/unic_langid_ffi_generated.h.stub
$(TOPOBJDIR)/intl/locale/unic_langid_ffi_generated.h: $(TOPOBJDIR)/intl/locale/$(MDDEPDIR)/unic_langid_ffi_generated.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/intl/locale/$(MDDEPDIR)/unic_langid_ffi_generated.h.pp
$(TOPOBJDIR)/intl/locale/$(MDDEPDIR)/unic_langid_ffi_generated.h.stub: /worker/build/build/RunCbindgen.py $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/intl/locale/rust/unic-langid-ffi
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate $(TOPOBJDIR)/intl/locale/unic_langid_ffi_generated.h $(TOPOBJDIR)/intl/locale/$(MDDEPDIR)/unic_langid_ffi_generated.h.pp $(TOPOBJDIR)/intl/locale/$(MDDEPDIR)/unic_langid_ffi_generated.h.stub $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/intl/locale/rust/unic-langid-ffi)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/intl/l10n/$(MDDEPDIR)/fluent_ffi_generated.h.stub
$(TOPOBJDIR)/intl/l10n/fluent_ffi_generated.h: $(TOPOBJDIR)/intl/l10n/$(MDDEPDIR)/fluent_ffi_generated.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/intl/l10n/$(MDDEPDIR)/fluent_ffi_generated.h.pp
$(TOPOBJDIR)/intl/l10n/$(MDDEPDIR)/fluent_ffi_generated.h.stub: /worker/build/build/RunCbindgen.py $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/intl/l10n/rust/fluent-ffi
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate $(TOPOBJDIR)/intl/l10n/fluent_ffi_generated.h $(TOPOBJDIR)/intl/l10n/$(MDDEPDIR)/fluent_ffi_generated.h.pp $(TOPOBJDIR)/intl/l10n/$(MDDEPDIR)/fluent_ffi_generated.h.stub $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/intl/l10n/rust/fluent-ffi)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/netwerk/base/mozurl/$(MDDEPDIR)/MozURL_ffi.h.stub
$(TOPOBJDIR)/netwerk/base/mozurl/MozURL_ffi.h: $(TOPOBJDIR)/netwerk/base/mozurl/$(MDDEPDIR)/MozURL_ffi.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/netwerk/base/mozurl/$(MDDEPDIR)/MozURL_ffi.h.pp
$(TOPOBJDIR)/netwerk/base/mozurl/$(MDDEPDIR)/MozURL_ffi.h.stub: /worker/build/build/RunCbindgen.py $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/netwerk/base/mozurl
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate $(TOPOBJDIR)/netwerk/base/mozurl/MozURL_ffi.h $(TOPOBJDIR)/netwerk/base/mozurl/$(MDDEPDIR)/MozURL_ffi.h.pp $(TOPOBJDIR)/netwerk/base/mozurl/$(MDDEPDIR)/MozURL_ffi.h.stub $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/netwerk/base/mozurl)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/netwerk/base/rust-helper/$(MDDEPDIR)/rust_helper.h.stub
$(TOPOBJDIR)/netwerk/base/rust-helper/rust_helper.h: $(TOPOBJDIR)/netwerk/base/rust-helper/$(MDDEPDIR)/rust_helper.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/netwerk/base/rust-helper/$(MDDEPDIR)/rust_helper.h.pp
$(TOPOBJDIR)/netwerk/base/rust-helper/$(MDDEPDIR)/rust_helper.h.stub: /worker/build/build/RunCbindgen.py $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/netwerk/base/rust-helper
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate $(TOPOBJDIR)/netwerk/base/rust-helper/rust_helper.h $(TOPOBJDIR)/netwerk/base/rust-helper/$(MDDEPDIR)/rust_helper.h.pp $(TOPOBJDIR)/netwerk/base/rust-helper/$(MDDEPDIR)/rust_helper.h.stub $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/netwerk/base/rust-helper)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/netwerk/dns/$(MDDEPDIR)/etld_data.inc.stub
$(TOPOBJDIR)/netwerk/dns/etld_data.inc: $(TOPOBJDIR)/netwerk/dns/$(MDDEPDIR)/etld_data.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/netwerk/dns/$(MDDEPDIR)/etld_data.inc.pp
$(TOPOBJDIR)/netwerk/dns/$(MDDEPDIR)/etld_data.inc.stub: /worker/build/netwerk/dns/prepare_tlds.py $(TOPSRCDIR)/netwerk/dns/effective_tld_names.dat
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/netwerk/dns/prepare_tlds.py main $(TOPOBJDIR)/netwerk/dns/etld_data.inc $(TOPOBJDIR)/netwerk/dns/$(MDDEPDIR)/etld_data.inc.pp $(TOPOBJDIR)/netwerk/dns/$(MDDEPDIR)/etld_data.inc.stub $(TOPSRCDIR)/netwerk/dns/effective_tld_names.dat)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/netwerk/dns/tests/unit/data/$(MDDEPDIR)/fake_remote_dafsa.bin.stub
$(TOPOBJDIR)/netwerk/dns/tests/unit/data/fake_remote_dafsa.bin: $(TOPOBJDIR)/netwerk/dns/tests/unit/data/$(MDDEPDIR)/fake_remote_dafsa.bin.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/netwerk/dns/tests/unit/data/$(MDDEPDIR)/fake_remote_dafsa.bin.pp
$(TOPOBJDIR)/netwerk/dns/tests/unit/data/$(MDDEPDIR)/fake_remote_dafsa.bin.stub: /worker/build/netwerk/dns/prepare_tlds.py $(TOPSRCDIR)/netwerk/dns/tests/unit/data/fake_public_suffix_list.dat
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/netwerk/dns/prepare_tlds.py main $(TOPOBJDIR)/netwerk/dns/tests/unit/data/fake_remote_dafsa.bin $(TOPOBJDIR)/netwerk/dns/tests/unit/data/$(MDDEPDIR)/fake_remote_dafsa.bin.pp $(TOPOBJDIR)/netwerk/dns/tests/unit/data/$(MDDEPDIR)/fake_remote_dafsa.bin.stub $(TOPSRCDIR)/netwerk/dns/tests/unit/data/fake_public_suffix_list.dat bin)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/netwerk/socket/neqo_glue/$(MDDEPDIR)/neqo_glue_ffi_generated.h.stub
$(TOPOBJDIR)/netwerk/socket/neqo_glue/neqo_glue_ffi_generated.h: $(TOPOBJDIR)/netwerk/socket/neqo_glue/$(MDDEPDIR)/neqo_glue_ffi_generated.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/netwerk/socket/neqo_glue/$(MDDEPDIR)/neqo_glue_ffi_generated.h.pp
$(TOPOBJDIR)/netwerk/socket/neqo_glue/$(MDDEPDIR)/neqo_glue_ffi_generated.h.stub: /worker/build/build/RunCbindgen.py $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/netwerk/socket/neqo_glue
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate $(TOPOBJDIR)/netwerk/socket/neqo_glue/neqo_glue_ffi_generated.h $(TOPOBJDIR)/netwerk/socket/neqo_glue/$(MDDEPDIR)/neqo_glue_ffi_generated.h.pp $(TOPOBJDIR)/netwerk/socket/neqo_glue/$(MDDEPDIR)/neqo_glue_ffi_generated.h.stub $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/netwerk/socket/neqo_glue)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/gfx/webrender_bindings/$(MDDEPDIR)/webrender_ffi_generated.h.stub
$(TOPOBJDIR)/gfx/webrender_bindings/webrender_ffi_generated.h: $(TOPOBJDIR)/gfx/webrender_bindings/$(MDDEPDIR)/webrender_ffi_generated.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/gfx/webrender_bindings/$(MDDEPDIR)/webrender_ffi_generated.h.pp
$(TOPOBJDIR)/gfx/webrender_bindings/$(MDDEPDIR)/webrender_ffi_generated.h.stub: /worker/build/build/RunCbindgen.py $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/gfx/webrender_bindings $(TOPSRCDIR)/gfx/wr/webrender $(TOPSRCDIR)/gfx/wr/webrender_api
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate $(TOPOBJDIR)/gfx/webrender_bindings/webrender_ffi_generated.h $(TOPOBJDIR)/gfx/webrender_bindings/$(MDDEPDIR)/webrender_ffi_generated.h.pp $(TOPOBJDIR)/gfx/webrender_bindings/$(MDDEPDIR)/webrender_ffi_generated.h.stub $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/gfx/webrender_bindings $(TOPSRCDIR)/gfx/wr/webrender $(TOPSRCDIR)/gfx/wr/webrender_api)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/gfx/wgpu_bindings/$(MDDEPDIR)/wgpu_ffi_generated.h.stub
$(TOPOBJDIR)/gfx/wgpu_bindings/wgpu_ffi_generated.h: $(TOPOBJDIR)/gfx/wgpu_bindings/$(MDDEPDIR)/wgpu_ffi_generated.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/gfx/wgpu_bindings/$(MDDEPDIR)/wgpu_ffi_generated.h.pp
$(TOPOBJDIR)/gfx/wgpu_bindings/$(MDDEPDIR)/wgpu_ffi_generated.h.stub: /worker/build/build/RunCbindgen.py $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/gfx/wgpu_bindings $(TOPSRCDIR)/gfx/wgpu
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate $(TOPOBJDIR)/gfx/wgpu_bindings/wgpu_ffi_generated.h $(TOPOBJDIR)/gfx/wgpu_bindings/$(MDDEPDIR)/wgpu_ffi_generated.h.pp $(TOPOBJDIR)/gfx/wgpu_bindings/$(MDDEPDIR)/wgpu_ffi_generated.h.stub $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/gfx/wgpu_bindings $(TOPSRCDIR)/gfx/wgpu)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/dom/base/$(MDDEPDIR)/UseCounterList.h.stub
$(TOPOBJDIR)/dom/base/UseCounterList.h: $(TOPOBJDIR)/dom/base/$(MDDEPDIR)/UseCounterList.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/dom/base/$(MDDEPDIR)/UseCounterList.h.pp
$(TOPOBJDIR)/dom/base/$(MDDEPDIR)/UseCounterList.h.stub: /worker/build/dom/base/gen-usecounters.py $(TOPSRCDIR)/dom/base/UseCounters.conf
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/dom/base/gen-usecounters.py use_counter_list $(TOPOBJDIR)/dom/base/UseCounterList.h $(TOPOBJDIR)/dom/base/$(MDDEPDIR)/UseCounterList.h.pp $(TOPOBJDIR)/dom/base/$(MDDEPDIR)/UseCounterList.h.stub $(TOPSRCDIR)/dom/base/UseCounters.conf)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/dom/base/$(MDDEPDIR)/UseCounterWorkerList.h.stub
$(TOPOBJDIR)/dom/base/UseCounterWorkerList.h: $(TOPOBJDIR)/dom/base/$(MDDEPDIR)/UseCounterWorkerList.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/dom/base/$(MDDEPDIR)/UseCounterWorkerList.h.pp
$(TOPOBJDIR)/dom/base/$(MDDEPDIR)/UseCounterWorkerList.h.stub: /worker/build/dom/base/gen-usecounters.py $(TOPSRCDIR)/dom/base/UseCountersWorker.conf
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/dom/base/gen-usecounters.py use_counter_list $(TOPOBJDIR)/dom/base/UseCounterWorkerList.h $(TOPOBJDIR)/dom/base/$(MDDEPDIR)/UseCounterWorkerList.h.pp $(TOPOBJDIR)/dom/base/$(MDDEPDIR)/UseCounterWorkerList.h.stub $(TOPSRCDIR)/dom/base/UseCountersWorker.conf)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/dom/bindings/$(MDDEPDIR)/CSS2Properties.webidl.stub
$(TOPOBJDIR)/dom/bindings/CSS2Properties.webidl: $(TOPOBJDIR)/dom/bindings/$(MDDEPDIR)/CSS2Properties.webidl.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/dom/bindings/$(MDDEPDIR)/CSS2Properties.webidl.pp
$(TOPOBJDIR)/dom/bindings/$(MDDEPDIR)/CSS2Properties.webidl.stub: /worker/build/dom/bindings/GenerateCSS2PropertiesWebIDL.py $(TOPSRCDIR)/dom/webidl/CSS2Properties.webidl.in $(TOPOBJDIR)/layout/style/ServoCSSPropList.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/dom/bindings/GenerateCSS2PropertiesWebIDL.py generate $(TOPOBJDIR)/dom/bindings/CSS2Properties.webidl $(TOPOBJDIR)/dom/bindings/$(MDDEPDIR)/CSS2Properties.webidl.pp $(TOPOBJDIR)/dom/bindings/$(MDDEPDIR)/CSS2Properties.webidl.stub $(TOPSRCDIR)/dom/webidl/CSS2Properties.webidl.in $(TOPOBJDIR)/layout/style/ServoCSSPropList.py)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/dom/media/$(MDDEPDIR)/audioipc_client_ffi_generated.h.stub
$(TOPOBJDIR)/dom/media/audioipc_client_ffi_generated.h: $(TOPOBJDIR)/dom/media/$(MDDEPDIR)/audioipc_client_ffi_generated.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/dom/media/$(MDDEPDIR)/audioipc_client_ffi_generated.h.pp
$(TOPOBJDIR)/dom/media/$(MDDEPDIR)/audioipc_client_ffi_generated.h.stub: /worker/build/build/RunCbindgen.py $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/third_party/rust/audioipc-client
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate $(TOPOBJDIR)/dom/media/audioipc_client_ffi_generated.h $(TOPOBJDIR)/dom/media/$(MDDEPDIR)/audioipc_client_ffi_generated.h.pp $(TOPOBJDIR)/dom/media/$(MDDEPDIR)/audioipc_client_ffi_generated.h.stub $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/third_party/rust/audioipc-client)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/dom/media/$(MDDEPDIR)/audioipc_server_ffi_generated.h.stub
$(TOPOBJDIR)/dom/media/audioipc_server_ffi_generated.h: $(TOPOBJDIR)/dom/media/$(MDDEPDIR)/audioipc_server_ffi_generated.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/dom/media/$(MDDEPDIR)/audioipc_server_ffi_generated.h.pp
$(TOPOBJDIR)/dom/media/$(MDDEPDIR)/audioipc_server_ffi_generated.h.stub: /worker/build/build/RunCbindgen.py $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/third_party/rust/audioipc-server
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate $(TOPOBJDIR)/dom/media/audioipc_server_ffi_generated.h $(TOPOBJDIR)/dom/media/$(MDDEPDIR)/audioipc_server_ffi_generated.h.pp $(TOPOBJDIR)/dom/media/$(MDDEPDIR)/audioipc_server_ffi_generated.h.stub $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/third_party/rust/audioipc-server)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/nsCSSPropertyID.h.stub
$(TOPOBJDIR)/layout/style/nsCSSPropertyID.h: $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/nsCSSPropertyID.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/nsCSSPropertyID.h.pp
$(TOPOBJDIR)/layout/style/$(MDDEPDIR)/nsCSSPropertyID.h.stub: /worker/build/layout/style/GenerateCSSPropertyID.py $(TOPSRCDIR)/layout/style/nsCSSPropertyID.h.in $(TOPOBJDIR)/layout/style/ServoCSSPropList.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/layout/style/GenerateCSSPropertyID.py generate $(TOPOBJDIR)/layout/style/nsCSSPropertyID.h $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/nsCSSPropertyID.h.pp $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/nsCSSPropertyID.h.stub $(TOPSRCDIR)/layout/style/nsCSSPropertyID.h.in $(TOPOBJDIR)/layout/style/ServoCSSPropList.py)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/ServoCSSPropList.h.stub
$(TOPOBJDIR)/layout/style/ServoCSSPropList.h: $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/ServoCSSPropList.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/ServoCSSPropList.h.pp
$(TOPOBJDIR)/layout/style/$(MDDEPDIR)/ServoCSSPropList.h.stub: /worker/build/layout/style/GenerateServoCSSPropList.py $(TOPOBJDIR)/layout/style/ServoCSSPropList.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/layout/style/GenerateServoCSSPropList.py generate_header $(TOPOBJDIR)/layout/style/ServoCSSPropList.h $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/ServoCSSPropList.h.pp $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/ServoCSSPropList.h.stub $(TOPOBJDIR)/layout/style/ServoCSSPropList.py)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/ServoCSSPropList.py.stub
$(TOPOBJDIR)/layout/style/ServoCSSPropList.py: $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/ServoCSSPropList.py.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/ServoCSSPropList.py.pp
$(TOPOBJDIR)/layout/style/$(MDDEPDIR)/ServoCSSPropList.py.stub: /worker/build/layout/style/GenerateServoCSSPropList.py $(TOPSRCDIR)/layout/style/ServoCSSPropList.mako.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/layout/style/GenerateServoCSSPropList.py generate_data $(TOPOBJDIR)/layout/style/ServoCSSPropList.py $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/ServoCSSPropList.py.pp $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/ServoCSSPropList.py.stub $(TOPSRCDIR)/layout/style/ServoCSSPropList.mako.py)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/CompositorAnimatableProperties.h.stub
$(TOPOBJDIR)/layout/style/CompositorAnimatableProperties.h: $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/CompositorAnimatableProperties.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/CompositorAnimatableProperties.h.pp
$(TOPOBJDIR)/layout/style/$(MDDEPDIR)/CompositorAnimatableProperties.h.stub: /worker/build/layout/style/GenerateCompositorAnimatableProperties.py $(TOPOBJDIR)/layout/style/ServoCSSPropList.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/layout/style/GenerateCompositorAnimatableProperties.py generate $(TOPOBJDIR)/layout/style/CompositorAnimatableProperties.h $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/CompositorAnimatableProperties.h.pp $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/CompositorAnimatableProperties.h.stub $(TOPOBJDIR)/layout/style/ServoCSSPropList.py)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/CountedUnknownProperties.h.stub
$(TOPOBJDIR)/layout/style/CountedUnknownProperties.h: $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/CountedUnknownProperties.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/CountedUnknownProperties.h.pp
$(TOPOBJDIR)/layout/style/$(MDDEPDIR)/CountedUnknownProperties.h.stub: /worker/build/layout/style/GenerateCountedUnknownProperties.py $(TOPSRCDIR)/servo/components/style/properties/counted_unknown_properties.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/layout/style/GenerateCountedUnknownProperties.py generate $(TOPOBJDIR)/layout/style/CountedUnknownProperties.h $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/CountedUnknownProperties.h.pp $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/CountedUnknownProperties.h.stub $(TOPSRCDIR)/servo/components/style/properties/counted_unknown_properties.py)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/nsComputedDOMStyleGenerated.inc.stub
$(TOPOBJDIR)/layout/style/nsComputedDOMStyleGenerated.inc: $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/nsComputedDOMStyleGenerated.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/nsComputedDOMStyleGenerated.inc.pp
$(TOPOBJDIR)/layout/style/$(MDDEPDIR)/nsComputedDOMStyleGenerated.inc.stub: /worker/build/layout/style/GenerateComputedDOMStyleGenerated.py $(TOPOBJDIR)/layout/style/ServoCSSPropList.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/layout/style/GenerateComputedDOMStyleGenerated.py generate $(TOPOBJDIR)/layout/style/nsComputedDOMStyleGenerated.inc $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/nsComputedDOMStyleGenerated.inc.pp $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/nsComputedDOMStyleGenerated.inc.stub $(TOPOBJDIR)/layout/style/ServoCSSPropList.py)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/nsCSSPropsGenerated.inc.stub
$(TOPOBJDIR)/layout/style/nsCSSPropsGenerated.inc: $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/nsCSSPropsGenerated.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/nsCSSPropsGenerated.inc.pp
$(TOPOBJDIR)/layout/style/$(MDDEPDIR)/nsCSSPropsGenerated.inc.stub: /worker/build/layout/style/GenerateCSSPropsGenerated.py $(TOPOBJDIR)/layout/style/ServoCSSPropList.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/layout/style/GenerateCSSPropsGenerated.py generate $(TOPOBJDIR)/layout/style/nsCSSPropsGenerated.inc $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/nsCSSPropsGenerated.inc.pp $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/nsCSSPropsGenerated.inc.stub $(TOPOBJDIR)/layout/style/ServoCSSPropList.py)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/ServoStyleConsts.h.stub
$(TOPOBJDIR)/layout/style/ServoStyleConsts.h: $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/ServoStyleConsts.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/ServoStyleConsts.h.pp
$(TOPOBJDIR)/layout/style/$(MDDEPDIR)/ServoStyleConsts.h.stub: /worker/build/build/RunCbindgen.py $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/servo/ports/geckolib $(TOPSRCDIR)/servo/components/style
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate $(TOPOBJDIR)/layout/style/ServoStyleConsts.h $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/ServoStyleConsts.h.pp $(TOPOBJDIR)/layout/style/$(MDDEPDIR)/ServoStyleConsts.h.stub $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/servo/ports/geckolib $(TOPSRCDIR)/servo/components/style)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/layout/style/test/$(MDDEPDIR)/css_properties.js.stub
$(TOPOBJDIR)/layout/style/test/css_properties.js: $(TOPOBJDIR)/layout/style/test/$(MDDEPDIR)/css_properties.js.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/layout/style/test/$(MDDEPDIR)/css_properties.js.pp
$(TOPOBJDIR)/layout/style/test/$(MDDEPDIR)/css_properties.js.stub: /worker/build/layout/style/test/gen-css-properties.py $(TOPSRCDIR)/layout/style/test/css_properties_like_longhand.js $(TOPOBJDIR)/layout/style/test/host_ListCSSProperties
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/layout/style/test/gen-css-properties.py main $(TOPOBJDIR)/layout/style/test/css_properties.js $(TOPOBJDIR)/layout/style/test/$(MDDEPDIR)/css_properties.js.pp $(TOPOBJDIR)/layout/style/test/$(MDDEPDIR)/css_properties.js.stub $(TOPSRCDIR)/layout/style/test/css_properties_like_longhand.js $(TOPOBJDIR)/layout/style/test/host_ListCSSProperties)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/layout/style/test/gtest/$(MDDEPDIR)/ExampleStylesheet.h.stub
$(TOPOBJDIR)/layout/style/test/gtest/ExampleStylesheet.h: $(TOPOBJDIR)/layout/style/test/gtest/$(MDDEPDIR)/ExampleStylesheet.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/layout/style/test/gtest/$(MDDEPDIR)/ExampleStylesheet.h.pp
$(TOPOBJDIR)/layout/style/test/gtest/$(MDDEPDIR)/ExampleStylesheet.h.stub: /worker/build/layout/style/test/gtest/generate_example_stylesheet.py $(TOPSRCDIR)/layout/style/test/gtest/example.css
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/layout/style/test/gtest/generate_example_stylesheet.py main $(TOPOBJDIR)/layout/style/test/gtest/ExampleStylesheet.h $(TOPOBJDIR)/layout/style/test/gtest/$(MDDEPDIR)/ExampleStylesheet.h.pp $(TOPOBJDIR)/layout/style/test/gtest/$(MDDEPDIR)/ExampleStylesheet.h.stub $(TOPSRCDIR)/layout/style/test/gtest/example.css)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/layout/generic/$(MDDEPDIR)/FrameIdList.h.stub
$(TOPOBJDIR)/layout/generic/FrameIdList.h: $(TOPOBJDIR)/layout/generic/$(MDDEPDIR)/FrameIdList.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/layout/generic/$(MDDEPDIR)/FrameIdList.h.pp
$(TOPOBJDIR)/layout/generic/$(MDDEPDIR)/FrameIdList.h.stub: /worker/build/layout/generic/GenerateFrameLists.py $(TOPSRCDIR)/layout/generic/FrameClasses.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/layout/generic/GenerateFrameLists.py generate_frame_id_list_h $(TOPOBJDIR)/layout/generic/FrameIdList.h $(TOPOBJDIR)/layout/generic/$(MDDEPDIR)/FrameIdList.h.pp $(TOPOBJDIR)/layout/generic/$(MDDEPDIR)/FrameIdList.h.stub $(TOPSRCDIR)/layout/generic/FrameClasses.py)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/layout/generic/$(MDDEPDIR)/FrameTypeList.h.stub
$(TOPOBJDIR)/layout/generic/FrameTypeList.h: $(TOPOBJDIR)/layout/generic/$(MDDEPDIR)/FrameTypeList.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/layout/generic/$(MDDEPDIR)/FrameTypeList.h.pp
$(TOPOBJDIR)/layout/generic/$(MDDEPDIR)/FrameTypeList.h.stub: /worker/build/layout/generic/GenerateFrameLists.py $(TOPSRCDIR)/layout/generic/FrameClasses.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/layout/generic/GenerateFrameLists.py generate_frame_type_list_h $(TOPOBJDIR)/layout/generic/FrameTypeList.h $(TOPOBJDIR)/layout/generic/$(MDDEPDIR)/FrameTypeList.h.pp $(TOPOBJDIR)/layout/generic/$(MDDEPDIR)/FrameTypeList.h.stub $(TOPSRCDIR)/layout/generic/FrameClasses.py)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/accessible/xpcom/$(MDDEPDIR)/xpcAccEvents.h.stub
$(TOPOBJDIR)/accessible/xpcom/xpcAccEvents.h: $(TOPOBJDIR)/accessible/xpcom/$(MDDEPDIR)/xpcAccEvents.h.stub ;
$(TOPOBJDIR)/accessible/xpcom/xpcAccEvents.cpp: $(TOPOBJDIR)/accessible/xpcom/$(MDDEPDIR)/xpcAccEvents.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/accessible/xpcom/$(MDDEPDIR)/xpcAccEvents.h.pp
$(TOPOBJDIR)/accessible/xpcom/$(MDDEPDIR)/xpcAccEvents.h.stub: /worker/build/accessible/xpcom/AccEventGen.py $(TOPSRCDIR)/accessible/xpcom/AccEvents.conf
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/accessible/xpcom/AccEventGen.py gen_files $(TOPOBJDIR)/accessible/xpcom/xpcAccEvents.h $(TOPOBJDIR)/accessible/xpcom/$(MDDEPDIR)/xpcAccEvents.h.pp $(TOPOBJDIR)/accessible/xpcom/$(MDDEPDIR)/xpcAccEvents.h.stub $(TOPSRCDIR)/accessible/xpcom/AccEvents.conf)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/manager/ssl/$(MDDEPDIR)/nsSTSPreloadListGenerated.inc.stub
$(TOPOBJDIR)/security/manager/ssl/nsSTSPreloadListGenerated.inc: $(TOPOBJDIR)/security/manager/ssl/$(MDDEPDIR)/nsSTSPreloadListGenerated.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/manager/ssl/$(MDDEPDIR)/nsSTSPreloadListGenerated.inc.pp
$(TOPOBJDIR)/security/manager/ssl/$(MDDEPDIR)/nsSTSPreloadListGenerated.inc.stub: /worker/build/xpcom/ds/tools/make_dafsa.py $(TOPSRCDIR)/security/manager/ssl/nsSTSPreloadList.inc
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/xpcom/ds/tools/make_dafsa.py main $(TOPOBJDIR)/security/manager/ssl/nsSTSPreloadListGenerated.inc $(TOPOBJDIR)/security/manager/ssl/$(MDDEPDIR)/nsSTSPreloadListGenerated.inc.pp $(TOPOBJDIR)/security/manager/ssl/$(MDDEPDIR)/nsSTSPreloadListGenerated.inc.stub $(TOPSRCDIR)/security/manager/ssl/nsSTSPreloadList.inc)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/manager/ssl/tests/unit/pkcs11testmodule/$(MDDEPDIR)/libpkcs11testmodule.so.symbols.stub
$(TOPOBJDIR)/security/manager/ssl/tests/unit/pkcs11testmodule/libpkcs11testmodule.so.symbols: $(TOPOBJDIR)/security/manager/ssl/tests/unit/pkcs11testmodule/$(MDDEPDIR)/libpkcs11testmodule.so.symbols.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/manager/ssl/tests/unit/pkcs11testmodule/$(MDDEPDIR)/libpkcs11testmodule.so.symbols.pp
$(TOPOBJDIR)/security/manager/ssl/tests/unit/pkcs11testmodule/$(MDDEPDIR)/libpkcs11testmodule.so.symbols.stub: /worker/build/python/mozbuild/mozbuild/action/generate_symbols_file.py $(TOPSRCDIR)/security/manager/ssl/tests/unit/pkcs11testmodule/pkcs11testmodule.symbols
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/generate_symbols_file.py generate_symbols_file $(TOPOBJDIR)/security/manager/ssl/tests/unit/pkcs11testmodule/libpkcs11testmodule.so.symbols $(TOPOBJDIR)/security/manager/ssl/tests/unit/pkcs11testmodule/$(MDDEPDIR)/libpkcs11testmodule.so.symbols.pp $(TOPOBJDIR)/security/manager/ssl/tests/unit/pkcs11testmodule/$(MDDEPDIR)/libpkcs11testmodule.so.symbols.stub $(TOPSRCDIR)/security/manager/ssl/tests/unit/pkcs11testmodule/pkcs11testmodule.symbols )
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/components/featuregates/$(MDDEPDIR)/feature_definitions.json.stub
$(TOPOBJDIR)/toolkit/components/featuregates/feature_definitions.json: $(TOPOBJDIR)/toolkit/components/featuregates/$(MDDEPDIR)/feature_definitions.json.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/components/featuregates/$(MDDEPDIR)/feature_definitions.json.pp
$(TOPOBJDIR)/toolkit/components/featuregates/$(MDDEPDIR)/feature_definitions.json.stub: /worker/build/toolkit/components/featuregates/gen_feature_definitions.py $(TOPSRCDIR)/toolkit/components/featuregates/Features.toml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/featuregates/gen_feature_definitions.py main $(TOPOBJDIR)/toolkit/components/featuregates/feature_definitions.json $(TOPOBJDIR)/toolkit/components/featuregates/$(MDDEPDIR)/feature_definitions.json.pp $(TOPOBJDIR)/toolkit/components/featuregates/$(MDDEPDIR)/feature_definitions.json.stub $(TOPSRCDIR)/toolkit/components/featuregates/Features.toml)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/fog_ffi_generated.h.stub
$(TOPOBJDIR)/toolkit/components/glean/fog_ffi_generated.h: $(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/fog_ffi_generated.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/fog_ffi_generated.h.pp
$(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/fog_ffi_generated.h.stub: /worker/build/build/RunCbindgen.py $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/toolkit/components/glean
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate $(TOPOBJDIR)/toolkit/components/glean/fog_ffi_generated.h $(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/fog_ffi_generated.h.pp $(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/fog_ffi_generated.h.stub $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/toolkit/components/glean)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/components/glean/api/src/$(MDDEPDIR)/metrics.rs.stub
$(TOPOBJDIR)/toolkit/components/glean/api/src/metrics.rs: $(TOPOBJDIR)/toolkit/components/glean/api/src/$(MDDEPDIR)/metrics.rs.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/components/glean/api/src/$(MDDEPDIR)/metrics.rs.pp
$(TOPOBJDIR)/toolkit/components/glean/api/src/$(MDDEPDIR)/metrics.rs.stub: /worker/build/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(TOPSRCDIR)/toolkit/components/glean/metrics_index.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/cpp.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/js.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/rust.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/string_table.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/util.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/metrics.yaml $(TOPSRCDIR)/toolkit/components/glean/test_metrics.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py main $(TOPOBJDIR)/toolkit/components/glean/api/src/metrics.rs $(TOPOBJDIR)/toolkit/components/glean/api/src/$(MDDEPDIR)/metrics.rs.pp $(TOPOBJDIR)/toolkit/components/glean/api/src/$(MDDEPDIR)/metrics.rs.stub $(TOPSRCDIR)/toolkit/components/glean/metrics_index.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/cpp.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/js.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/rust.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/string_table.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/util.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/metrics.yaml $(TOPSRCDIR)/toolkit/components/glean/test_metrics.yaml 87.0)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/GleanMetrics.h.stub
$(TOPOBJDIR)/toolkit/components/glean/GleanMetrics.h: $(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/GleanMetrics.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/GleanMetrics.h.pp
$(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/GleanMetrics.h.stub: /worker/build/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(TOPSRCDIR)/toolkit/components/glean/metrics_index.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/cpp.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/js.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/rust.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/string_table.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/util.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/metrics.yaml $(TOPSRCDIR)/toolkit/components/glean/test_metrics.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py cpp_metrics $(TOPOBJDIR)/toolkit/components/glean/GleanMetrics.h $(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/GleanMetrics.h.pp $(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/GleanMetrics.h.stub $(TOPSRCDIR)/toolkit/components/glean/metrics_index.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/cpp.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/js.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/rust.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/string_table.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/util.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/metrics.yaml $(TOPSRCDIR)/toolkit/components/glean/test_metrics.yaml 87.0)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/GleanJSMetricsLookup.h.stub
$(TOPOBJDIR)/toolkit/components/glean/GleanJSMetricsLookup.h: $(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/GleanJSMetricsLookup.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/GleanJSMetricsLookup.h.pp
$(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/GleanJSMetricsLookup.h.stub: /worker/build/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(TOPSRCDIR)/toolkit/components/glean/metrics_index.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/cpp.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/js.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/rust.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/string_table.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/util.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/metrics.yaml $(TOPSRCDIR)/toolkit/components/glean/test_metrics.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py js_metrics $(TOPOBJDIR)/toolkit/components/glean/GleanJSMetricsLookup.h $(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/GleanJSMetricsLookup.h.pp $(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/GleanJSMetricsLookup.h.stub $(TOPSRCDIR)/toolkit/components/glean/metrics_index.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/cpp.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/js.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/rust.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/string_table.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/util.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/metrics.yaml $(TOPSRCDIR)/toolkit/components/glean/test_metrics.yaml 87.0)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/components/glean/api/src/$(MDDEPDIR)/pings.rs.stub
$(TOPOBJDIR)/toolkit/components/glean/api/src/pings.rs: $(TOPOBJDIR)/toolkit/components/glean/api/src/$(MDDEPDIR)/pings.rs.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/components/glean/api/src/$(MDDEPDIR)/pings.rs.pp
$(TOPOBJDIR)/toolkit/components/glean/api/src/$(MDDEPDIR)/pings.rs.stub: /worker/build/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(TOPSRCDIR)/toolkit/components/glean/metrics_index.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/cpp.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/js.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/rust.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/string_table.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/util.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/pings.yaml $(TOPSRCDIR)/toolkit/components/glean/test_pings.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py main $(TOPOBJDIR)/toolkit/components/glean/api/src/pings.rs $(TOPOBJDIR)/toolkit/components/glean/api/src/$(MDDEPDIR)/pings.rs.pp $(TOPOBJDIR)/toolkit/components/glean/api/src/$(MDDEPDIR)/pings.rs.stub $(TOPSRCDIR)/toolkit/components/glean/metrics_index.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/cpp.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/js.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/rust.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/string_table.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/util.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/pings.yaml $(TOPSRCDIR)/toolkit/components/glean/test_pings.yaml 87.0)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/GleanPings.h.stub
$(TOPOBJDIR)/toolkit/components/glean/GleanPings.h: $(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/GleanPings.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/GleanPings.h.pp
$(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/GleanPings.h.stub: /worker/build/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(TOPSRCDIR)/toolkit/components/glean/metrics_index.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/cpp.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/js.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/rust.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/string_table.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/util.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/pings.yaml $(TOPSRCDIR)/toolkit/components/glean/test_pings.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py cpp_metrics $(TOPOBJDIR)/toolkit/components/glean/GleanPings.h $(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/GleanPings.h.pp $(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/GleanPings.h.stub $(TOPSRCDIR)/toolkit/components/glean/metrics_index.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/cpp.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/js.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/rust.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/string_table.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/util.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/pings.yaml $(TOPSRCDIR)/toolkit/components/glean/test_pings.yaml 87.0)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/GleanJSPingsLookup.h.stub
$(TOPOBJDIR)/toolkit/components/glean/GleanJSPingsLookup.h: $(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/GleanJSPingsLookup.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/GleanJSPingsLookup.h.pp
$(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/GleanJSPingsLookup.h.stub: /worker/build/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(TOPSRCDIR)/toolkit/components/glean/metrics_index.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/cpp.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/js.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/rust.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/string_table.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/util.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/pings.yaml $(TOPSRCDIR)/toolkit/components/glean/test_pings.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py js_metrics $(TOPOBJDIR)/toolkit/components/glean/GleanJSPingsLookup.h $(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/GleanJSPingsLookup.h.pp $(TOPOBJDIR)/toolkit/components/glean/$(MDDEPDIR)/GleanJSPingsLookup.h.stub $(TOPSRCDIR)/toolkit/components/glean/metrics_index.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/cpp.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/js.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/rust.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/string_table.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/util.py $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust.jinja2 $(TOPSRCDIR)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust_pings.jinja2 $(TOPSRCDIR)/toolkit/components/glean/pings.yaml $(TOPSRCDIR)/toolkit/components/glean/test_pings.yaml 87.0)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramData.inc.stub
$(TOPOBJDIR)/toolkit/components/telemetry/TelemetryHistogramData.inc: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramData.inc.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramData.inc.pp
$(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramData.inc.stub: /worker/build/toolkit/components/telemetry/build_scripts/gen_histogram_data.py $(TOPSRCDIR)/toolkit/components/telemetry/Histograms.json $(TOPSRCDIR)/dom/base/UseCounters.conf $(TOPSRCDIR)/dom/base/nsDeprecatedOperationList.h $(TOPOBJDIR)/layout/style/ServoCSSPropList.py $(TOPSRCDIR)/servo/components/style/properties/counted_unknown_properties.py $(TOPSRCDIR)/dom/base/UseCountersWorker.conf
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/telemetry/build_scripts/gen_histogram_data.py main $(TOPOBJDIR)/toolkit/components/telemetry/TelemetryHistogramData.inc $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramData.inc.pp $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramData.inc.stub $(TOPSRCDIR)/toolkit/components/telemetry/Histograms.json $(TOPSRCDIR)/dom/base/UseCounters.conf $(TOPSRCDIR)/dom/base/nsDeprecatedOperationList.h $(TOPOBJDIR)/layout/style/ServoCSSPropList.py $(TOPSRCDIR)/servo/components/style/properties/counted_unknown_properties.py $(TOPSRCDIR)/dom/base/UseCountersWorker.conf)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramEnums.h.stub
$(TOPOBJDIR)/toolkit/components/telemetry/TelemetryHistogramEnums.h: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramEnums.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramEnums.h.pp
$(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramEnums.h.stub: /worker/build/toolkit/components/telemetry/build_scripts/gen_histogram_enum.py $(TOPSRCDIR)/toolkit/components/telemetry/Histograms.json $(TOPSRCDIR)/dom/base/UseCounters.conf $(TOPSRCDIR)/dom/base/nsDeprecatedOperationList.h $(TOPOBJDIR)/layout/style/ServoCSSPropList.py $(TOPSRCDIR)/servo/components/style/properties/counted_unknown_properties.py $(TOPSRCDIR)/dom/base/UseCountersWorker.conf
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/telemetry/build_scripts/gen_histogram_enum.py main $(TOPOBJDIR)/toolkit/components/telemetry/TelemetryHistogramEnums.h $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramEnums.h.pp $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramEnums.h.stub $(TOPSRCDIR)/toolkit/components/telemetry/Histograms.json $(TOPSRCDIR)/dom/base/UseCounters.conf $(TOPSRCDIR)/dom/base/nsDeprecatedOperationList.h $(TOPOBJDIR)/layout/style/ServoCSSPropList.py $(TOPSRCDIR)/servo/components/style/properties/counted_unknown_properties.py $(TOPSRCDIR)/dom/base/UseCountersWorker.conf)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramNameMap.h.stub
$(TOPOBJDIR)/toolkit/components/telemetry/TelemetryHistogramNameMap.h: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramNameMap.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramNameMap.h.pp
$(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramNameMap.h.stub: /worker/build/toolkit/components/telemetry/build_scripts/gen_histogram_phf.py $(TOPSRCDIR)/toolkit/components/telemetry/Histograms.json $(TOPSRCDIR)/dom/base/UseCounters.conf $(TOPSRCDIR)/dom/base/nsDeprecatedOperationList.h $(TOPOBJDIR)/layout/style/ServoCSSPropList.py $(TOPSRCDIR)/servo/components/style/properties/counted_unknown_properties.py $(TOPSRCDIR)/dom/base/UseCountersWorker.conf
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/telemetry/build_scripts/gen_histogram_phf.py main $(TOPOBJDIR)/toolkit/components/telemetry/TelemetryHistogramNameMap.h $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramNameMap.h.pp $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramNameMap.h.stub $(TOPSRCDIR)/toolkit/components/telemetry/Histograms.json $(TOPSRCDIR)/dom/base/UseCounters.conf $(TOPSRCDIR)/dom/base/nsDeprecatedOperationList.h $(TOPOBJDIR)/layout/style/ServoCSSPropList.py $(TOPSRCDIR)/servo/components/style/properties/counted_unknown_properties.py $(TOPSRCDIR)/dom/base/UseCountersWorker.conf)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryScalarData.h.stub
$(TOPOBJDIR)/toolkit/components/telemetry/TelemetryScalarData.h: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryScalarData.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryScalarData.h.pp
$(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryScalarData.h.stub: /worker/build/toolkit/components/telemetry/build_scripts/gen_scalar_data.py $(TOPSRCDIR)/toolkit/components/telemetry/Scalars.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/telemetry/build_scripts/gen_scalar_data.py main $(TOPOBJDIR)/toolkit/components/telemetry/TelemetryScalarData.h $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryScalarData.h.pp $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryScalarData.h.stub $(TOPSRCDIR)/toolkit/components/telemetry/Scalars.yaml)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryScalarEnums.h.stub
$(TOPOBJDIR)/toolkit/components/telemetry/TelemetryScalarEnums.h: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryScalarEnums.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryScalarEnums.h.pp
$(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryScalarEnums.h.stub: /worker/build/toolkit/components/telemetry/build_scripts/gen_scalar_enum.py $(TOPSRCDIR)/toolkit/components/telemetry/Scalars.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/telemetry/build_scripts/gen_scalar_enum.py main $(TOPOBJDIR)/toolkit/components/telemetry/TelemetryScalarEnums.h $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryScalarEnums.h.pp $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryScalarEnums.h.stub $(TOPSRCDIR)/toolkit/components/telemetry/Scalars.yaml)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/ScalarArtifactDefinitions.json.stub
$(TOPOBJDIR)/toolkit/components/telemetry/ScalarArtifactDefinitions.json: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/ScalarArtifactDefinitions.json.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/ScalarArtifactDefinitions.json.pp
$(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/ScalarArtifactDefinitions.json.stub: /worker/build/toolkit/components/telemetry/build_scripts/gen_scalar_data.py $(TOPSRCDIR)/toolkit/components/telemetry/Scalars.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/telemetry/build_scripts/gen_scalar_data.py generate_JSON_definitions $(TOPOBJDIR)/toolkit/components/telemetry/ScalarArtifactDefinitions.json $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/ScalarArtifactDefinitions.json.pp $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/ScalarArtifactDefinitions.json.stub $(TOPSRCDIR)/toolkit/components/telemetry/Scalars.yaml)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryEventData.h.stub
$(TOPOBJDIR)/toolkit/components/telemetry/TelemetryEventData.h: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryEventData.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryEventData.h.pp
$(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryEventData.h.stub: /worker/build/toolkit/components/telemetry/build_scripts/gen_event_data.py $(TOPSRCDIR)/toolkit/components/telemetry/Events.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/telemetry/build_scripts/gen_event_data.py main $(TOPOBJDIR)/toolkit/components/telemetry/TelemetryEventData.h $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryEventData.h.pp $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryEventData.h.stub $(TOPSRCDIR)/toolkit/components/telemetry/Events.yaml)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryEventEnums.h.stub
$(TOPOBJDIR)/toolkit/components/telemetry/TelemetryEventEnums.h: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryEventEnums.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryEventEnums.h.pp
$(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryEventEnums.h.stub: /worker/build/toolkit/components/telemetry/build_scripts/gen_event_enum.py $(TOPSRCDIR)/toolkit/components/telemetry/Events.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/telemetry/build_scripts/gen_event_enum.py main $(TOPOBJDIR)/toolkit/components/telemetry/TelemetryEventEnums.h $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryEventEnums.h.pp $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryEventEnums.h.stub $(TOPSRCDIR)/toolkit/components/telemetry/Events.yaml)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/EventArtifactDefinitions.json.stub
$(TOPOBJDIR)/toolkit/components/telemetry/EventArtifactDefinitions.json: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/EventArtifactDefinitions.json.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/EventArtifactDefinitions.json.pp
$(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/EventArtifactDefinitions.json.stub: /worker/build/toolkit/components/telemetry/build_scripts/gen_event_data.py $(TOPSRCDIR)/toolkit/components/telemetry/Events.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/telemetry/build_scripts/gen_event_data.py generate_JSON_definitions $(TOPOBJDIR)/toolkit/components/telemetry/EventArtifactDefinitions.json $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/EventArtifactDefinitions.json.pp $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/EventArtifactDefinitions.json.stub $(TOPSRCDIR)/toolkit/components/telemetry/Events.yaml)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryProcessEnums.h.stub
$(TOPOBJDIR)/toolkit/components/telemetry/TelemetryProcessEnums.h: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryProcessEnums.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryProcessEnums.h.pp
$(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryProcessEnums.h.stub: /worker/build/toolkit/components/telemetry/build_scripts/gen_process_enum.py $(TOPSRCDIR)/toolkit/components/telemetry/Processes.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/telemetry/build_scripts/gen_process_enum.py main $(TOPOBJDIR)/toolkit/components/telemetry/TelemetryProcessEnums.h $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryProcessEnums.h.pp $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryProcessEnums.h.stub $(TOPSRCDIR)/toolkit/components/telemetry/Processes.yaml)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryProcessData.h.stub
$(TOPOBJDIR)/toolkit/components/telemetry/TelemetryProcessData.h: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryProcessData.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryProcessData.h.pp
$(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryProcessData.h.stub: /worker/build/toolkit/components/telemetry/build_scripts/gen_process_data.py $(TOPSRCDIR)/toolkit/components/telemetry/Processes.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/telemetry/build_scripts/gen_process_data.py main $(TOPOBJDIR)/toolkit/components/telemetry/TelemetryProcessData.h $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryProcessData.h.pp $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryProcessData.h.stub $(TOPSRCDIR)/toolkit/components/telemetry/Processes.yaml)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/glean_checks.stub
$(TOPOBJDIR)/toolkit/components/telemetry/glean_checks: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/glean_checks.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/glean_checks.pp
$(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/glean_checks.stub: /worker/build/toolkit/components/telemetry/build_scripts/run_glean_parser.py $(TOPSRCDIR)/toolkit/components/telemetry/geckoview/streaming/metrics.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/telemetry/build_scripts/run_glean_parser.py main $(TOPOBJDIR)/toolkit/components/telemetry/glean_checks $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/glean_checks.pp $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/glean_checks.stub $(TOPSRCDIR)/toolkit/components/telemetry/geckoview/streaming/metrics.yaml)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryUserInteractionData.h.stub
$(TOPOBJDIR)/toolkit/components/telemetry/TelemetryUserInteractionData.h: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryUserInteractionData.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryUserInteractionData.h.pp
$(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryUserInteractionData.h.stub: /worker/build/toolkit/components/telemetry/build_scripts/gen_userinteraction_data.py $(TOPSRCDIR)/toolkit/components/telemetry/UserInteractions.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/telemetry/build_scripts/gen_userinteraction_data.py main $(TOPOBJDIR)/toolkit/components/telemetry/TelemetryUserInteractionData.h $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryUserInteractionData.h.pp $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryUserInteractionData.h.stub $(TOPSRCDIR)/toolkit/components/telemetry/UserInteractions.yaml)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryUserInteractionNameMap.h.stub
$(TOPOBJDIR)/toolkit/components/telemetry/TelemetryUserInteractionNameMap.h: $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryUserInteractionNameMap.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryUserInteractionNameMap.h.pp
$(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryUserInteractionNameMap.h.stub: /worker/build/toolkit/components/telemetry/build_scripts/gen_userinteraction_phf.py $(TOPSRCDIR)/toolkit/components/telemetry/UserInteractions.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/telemetry/build_scripts/gen_userinteraction_phf.py main $(TOPOBJDIR)/toolkit/components/telemetry/TelemetryUserInteractionNameMap.h $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryUserInteractionNameMap.h.pp $(TOPOBJDIR)/toolkit/components/telemetry/$(MDDEPDIR)/TelemetryUserInteractionNameMap.h.stub $(TOPSRCDIR)/toolkit/components/telemetry/UserInteractions.yaml)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-a-1.0.xpi.stub
$(TOPOBJDIR)/toolkit/components/normandy/test/browser/addons/normandydriver-a-1.0.xpi: $(TOPOBJDIR)/toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-a-1.0.xpi.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-a-1.0.xpi.pp
$(TOPOBJDIR)/toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-a-1.0.xpi.stub: /worker/build/toolkit/components/normandy/test/create_xpi.py $(TOPSRCDIR)/toolkit/components/normandy/test/browser/addons/normandydriver-a-1.0
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/normandy/test/create_xpi.py main $(TOPOBJDIR)/toolkit/components/normandy/test/browser/addons/normandydriver-a-1.0.xpi $(TOPOBJDIR)/toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-a-1.0.xpi.pp $(TOPOBJDIR)/toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-a-1.0.xpi.stub $(TOPSRCDIR)/toolkit/components/normandy/test/browser/addons/normandydriver-a-1.0)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-b-1.0.xpi.stub
$(TOPOBJDIR)/toolkit/components/normandy/test/browser/addons/normandydriver-b-1.0.xpi: $(TOPOBJDIR)/toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-b-1.0.xpi.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-b-1.0.xpi.pp
$(TOPOBJDIR)/toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-b-1.0.xpi.stub: /worker/build/toolkit/components/normandy/test/create_xpi.py $(TOPSRCDIR)/toolkit/components/normandy/test/browser/addons/normandydriver-b-1.0
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/normandy/test/create_xpi.py main $(TOPOBJDIR)/toolkit/components/normandy/test/browser/addons/normandydriver-b-1.0.xpi $(TOPOBJDIR)/toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-b-1.0.xpi.pp $(TOPOBJDIR)/toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-b-1.0.xpi.stub $(TOPSRCDIR)/toolkit/components/normandy/test/browser/addons/normandydriver-b-1.0)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-a-2.0.xpi.stub
$(TOPOBJDIR)/toolkit/components/normandy/test/browser/addons/normandydriver-a-2.0.xpi: $(TOPOBJDIR)/toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-a-2.0.xpi.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-a-2.0.xpi.pp
$(TOPOBJDIR)/toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-a-2.0.xpi.stub: /worker/build/toolkit/components/normandy/test/create_xpi.py $(TOPSRCDIR)/toolkit/components/normandy/test/browser/addons/normandydriver-a-2.0
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/normandy/test/create_xpi.py main $(TOPOBJDIR)/toolkit/components/normandy/test/browser/addons/normandydriver-a-2.0.xpi $(TOPOBJDIR)/toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-a-2.0.xpi.pp $(TOPOBJDIR)/toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-a-2.0.xpi.stub $(TOPSRCDIR)/toolkit/components/normandy/test/browser/addons/normandydriver-a-2.0)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/crashreporter/$(MDDEPDIR)/CrashAnnotations.h.stub
$(TOPOBJDIR)/toolkit/crashreporter/CrashAnnotations.h: $(TOPOBJDIR)/toolkit/crashreporter/$(MDDEPDIR)/CrashAnnotations.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/crashreporter/$(MDDEPDIR)/CrashAnnotations.h.pp
$(TOPOBJDIR)/toolkit/crashreporter/$(MDDEPDIR)/CrashAnnotations.h.stub: /worker/build/toolkit/crashreporter/generate_crash_reporter_sources.py $(TOPSRCDIR)/toolkit/crashreporter/CrashAnnotations.h.in $(TOPSRCDIR)/toolkit/crashreporter/CrashAnnotations.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/crashreporter/generate_crash_reporter_sources.py emit_header $(TOPOBJDIR)/toolkit/crashreporter/CrashAnnotations.h $(TOPOBJDIR)/toolkit/crashreporter/$(MDDEPDIR)/CrashAnnotations.h.pp $(TOPOBJDIR)/toolkit/crashreporter/$(MDDEPDIR)/CrashAnnotations.h.stub $(TOPSRCDIR)/toolkit/crashreporter/CrashAnnotations.h.in $(TOPSRCDIR)/toolkit/crashreporter/CrashAnnotations.yaml)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/crashreporter/rust_minidump_writer_linux/$(MDDEPDIR)/rust_minidump_writer_linux_ffi_generated.h.stub
$(TOPOBJDIR)/toolkit/crashreporter/rust_minidump_writer_linux/rust_minidump_writer_linux_ffi_generated.h: $(TOPOBJDIR)/toolkit/crashreporter/rust_minidump_writer_linux/$(MDDEPDIR)/rust_minidump_writer_linux_ffi_generated.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/crashreporter/rust_minidump_writer_linux/$(MDDEPDIR)/rust_minidump_writer_linux_ffi_generated.h.pp
$(TOPOBJDIR)/toolkit/crashreporter/rust_minidump_writer_linux/$(MDDEPDIR)/rust_minidump_writer_linux_ffi_generated.h.stub: /worker/build/build/RunCbindgen.py $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/toolkit/crashreporter/rust_minidump_writer_linux
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate $(TOPOBJDIR)/toolkit/crashreporter/rust_minidump_writer_linux/rust_minidump_writer_linux_ffi_generated.h $(TOPOBJDIR)/toolkit/crashreporter/rust_minidump_writer_linux/$(MDDEPDIR)/rust_minidump_writer_linux_ffi_generated.h.pp $(TOPOBJDIR)/toolkit/crashreporter/rust_minidump_writer_linux/$(MDDEPDIR)/rust_minidump_writer_linux_ffi_generated.h.stub $(TOPOBJDIR)/config/cbindgen-metadata.json $(TOPSRCDIR)/toolkit/crashreporter/rust_minidump_writer_linux)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/locales/$(MDDEPDIR)/multilocale.txt.stub
$(TOPOBJDIR)/toolkit/locales/multilocale.txt: $(TOPOBJDIR)/toolkit/locales/$(MDDEPDIR)/multilocale.txt.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/locales/$(MDDEPDIR)/multilocale.txt.pp
$(TOPOBJDIR)/toolkit/locales/$(MDDEPDIR)/multilocale.txt.stub: /worker/build/toolkit/locales/gen_multilocale.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/locales/gen_multilocale.py main $(TOPOBJDIR)/toolkit/locales/multilocale.txt $(TOPOBJDIR)/toolkit/locales/$(MDDEPDIR)/multilocale.txt.pp $(TOPOBJDIR)/toolkit/locales/$(MDDEPDIR)/multilocale.txt.stub)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/locales/$(MDDEPDIR)/update.locale.stub
$(TOPOBJDIR)/toolkit/locales/update.locale: $(TOPOBJDIR)/toolkit/locales/$(MDDEPDIR)/update.locale.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/locales/$(MDDEPDIR)/update.locale.pp
$(TOPOBJDIR)/toolkit/locales/$(MDDEPDIR)/update.locale.stub: /worker/build/toolkit/locales/generate_update_locale.py $(if $(IS_LANGUAGE_REPACK),FORCE)
	$(REPORT_BUILD)
	$(call py_action,file_generate,--locale=$(AB_CD) /worker/build/toolkit/locales/generate_update_locale.py main $(TOPOBJDIR)/toolkit/locales/update.locale $(TOPOBJDIR)/toolkit/locales/$(MDDEPDIR)/update.locale.pp $(TOPOBJDIR)/toolkit/locales/$(MDDEPDIR)/update.locale.stub)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/locales/$(MDDEPDIR)/locale.ini.stub
$(TOPOBJDIR)/toolkit/locales/locale.ini: $(TOPOBJDIR)/toolkit/locales/$(MDDEPDIR)/locale.ini.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/locales/$(MDDEPDIR)/locale.ini.pp
$(TOPOBJDIR)/toolkit/locales/$(MDDEPDIR)/locale.ini.stub: /worker/build/toolkit/locales/generate_locale_ini.py $(if $(IS_LANGUAGE_REPACK),FORCE)
	$(REPORT_BUILD)
	$(call py_action,file_generate,--locale=$(AB_CD) /worker/build/toolkit/locales/generate_locale_ini.py main $(TOPOBJDIR)/toolkit/locales/locale.ini $(TOPOBJDIR)/toolkit/locales/$(MDDEPDIR)/locale.ini.pp $(TOPOBJDIR)/toolkit/locales/$(MDDEPDIR)/locale.ini.stub)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/mozapps/extensions/$(MDDEPDIR)/built_in_addons.json.stub
$(TOPOBJDIR)/toolkit/mozapps/extensions/built_in_addons.json: $(TOPOBJDIR)/toolkit/mozapps/extensions/$(MDDEPDIR)/built_in_addons.json.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/mozapps/extensions/$(MDDEPDIR)/built_in_addons.json.pp
$(TOPOBJDIR)/toolkit/mozapps/extensions/$(MDDEPDIR)/built_in_addons.json.stub: /worker/build/toolkit/mozapps/extensions/gen_built_in_addons.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/extensions/gen_built_in_addons.py main $(TOPOBJDIR)/toolkit/mozapps/extensions/built_in_addons.json $(TOPOBJDIR)/toolkit/mozapps/extensions/$(MDDEPDIR)/built_in_addons.json.pp $(TOPOBJDIR)/toolkit/mozapps/extensions/$(MDDEPDIR)/built_in_addons.json.stub --features=browser/features)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop1.xpi.stub
$(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop1.xpi: $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop1.xpi.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop1.xpi.pp
$(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop1.xpi.stub: /worker/build/toolkit/mozapps/extensions/test/create_xpi.py $(TOPSRCDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop1
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/extensions/test/create_xpi.py main $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop1.xpi $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop1.xpi.pp $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop1.xpi.stub $(TOPSRCDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop1)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop1.zip.stub
$(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop1.zip: $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop1.zip.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop1.zip.pp
$(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop1.zip.stub: /worker/build/toolkit/mozapps/extensions/test/create_xpi.py $(TOPSRCDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop1
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/extensions/test/create_xpi.py main $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop1.zip $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop1.zip.pp $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop1.zip.stub $(TOPSRCDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop1)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop2.xpi.stub
$(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop2.xpi: $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop2.xpi.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop2.xpi.pp
$(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop2.xpi.stub: /worker/build/toolkit/mozapps/extensions/test/create_xpi.py $(TOPSRCDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop2
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/extensions/test/create_xpi.py main $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop2.xpi $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop2.xpi.pp $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop2.xpi.stub $(TOPSRCDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop2)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop2.zip.stub
$(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop2.zip: $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop2.zip.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop2.zip.pp
$(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop2.zip.stub: /worker/build/toolkit/mozapps/extensions/test/create_xpi.py $(TOPSRCDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop2
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/extensions/test/create_xpi.py main $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop2.zip $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop2.zip.pp $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop2.zip.stub $(TOPSRCDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop2)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop_incompat.xpi.stub
$(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop_incompat.xpi: $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop_incompat.xpi.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop_incompat.xpi.pp
$(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop_incompat.xpi.stub: /worker/build/toolkit/mozapps/extensions/test/create_xpi.py $(TOPSRCDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop_incompat
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/extensions/test/create_xpi.py main $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop_incompat.xpi $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop_incompat.xpi.pp $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop_incompat.xpi.stub $(TOPSRCDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop_incompat)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop_incompat.zip.stub
$(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop_incompat.zip: $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop_incompat.zip.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop_incompat.zip.pp
$(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop_incompat.zip.stub: /worker/build/toolkit/mozapps/extensions/test/create_xpi.py $(TOPSRCDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop_incompat
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/extensions/test/create_xpi.py main $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop_incompat.zip $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop_incompat.zip.pp $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop_incompat.zip.stub $(TOPSRCDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop_incompat)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_installssl.xpi.stub
$(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_installssl.xpi: $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_installssl.xpi.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_installssl.xpi.pp
$(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_installssl.xpi.stub: /worker/build/toolkit/mozapps/extensions/test/create_xpi.py $(TOPSRCDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_installssl
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/extensions/test/create_xpi.py main $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_installssl.xpi $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_installssl.xpi.pp $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_installssl.xpi.stub $(TOPSRCDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_installssl)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_installssl.zip.stub
$(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_installssl.zip: $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_installssl.zip.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_installssl.zip.pp
$(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_installssl.zip.stub: /worker/build/toolkit/mozapps/extensions/test/create_xpi.py $(TOPSRCDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_installssl
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/extensions/test/create_xpi.py main $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_installssl.zip $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_installssl.zip.pp $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_installssl.zip.stub $(TOPSRCDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_installssl)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_theme.xpi.stub
$(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_theme.xpi: $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_theme.xpi.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_theme.xpi.pp
$(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_theme.xpi.stub: /worker/build/toolkit/mozapps/extensions/test/create_xpi.py $(TOPSRCDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_theme
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/extensions/test/create_xpi.py main $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_theme.xpi $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_theme.xpi.pp $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_theme.xpi.stub $(TOPSRCDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_theme)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_theme.zip.stub
$(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_theme.zip: $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_theme.zip.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_theme.zip.pp
$(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_theme.zip.stub: /worker/build/toolkit/mozapps/extensions/test/create_xpi.py $(TOPSRCDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_theme
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/extensions/test/create_xpi.py main $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_theme.zip $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_theme.zip.pp $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_theme.zip.stub $(TOPSRCDIR)/toolkit/mozapps/extensions/test/browser/addons/browser_theme)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/options_signed.xpi.stub
$(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/options_signed.xpi: $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/options_signed.xpi.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/options_signed.xpi.pp
$(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/options_signed.xpi.stub: /worker/build/toolkit/mozapps/extensions/test/create_xpi.py $(TOPSRCDIR)/toolkit/mozapps/extensions/test/browser/addons/options_signed
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/extensions/test/create_xpi.py main $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/options_signed.xpi $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/options_signed.xpi.pp $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/options_signed.xpi.stub $(TOPSRCDIR)/toolkit/mozapps/extensions/test/browser/addons/options_signed)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/options_signed.zip.stub
$(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/options_signed.zip: $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/options_signed.zip.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/options_signed.zip.pp
$(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/options_signed.zip.stub: /worker/build/toolkit/mozapps/extensions/test/create_xpi.py $(TOPSRCDIR)/toolkit/mozapps/extensions/test/browser/addons/options_signed
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/extensions/test/create_xpi.py main $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/options_signed.zip $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/options_signed.zip.pp $(TOPOBJDIR)/toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/options_signed.zip.stub $(TOPSRCDIR)/toolkit/mozapps/extensions/test/browser/addons/options_signed)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/dep1Cert.h.stub
$(TOPOBJDIR)/toolkit/mozapps/update/updater/dep1Cert.h: $(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/dep1Cert.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/dep1Cert.h.pp
$(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/dep1Cert.h.stub: /worker/build/toolkit/mozapps/update/updater/gen_cert_header.py $(TOPSRCDIR)/toolkit/mozapps/update/updater/dep1.der
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/update/updater/gen_cert_header.py create_header $(TOPOBJDIR)/toolkit/mozapps/update/updater/dep1Cert.h $(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/dep1Cert.h.pp $(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/dep1Cert.h.stub $(TOPSRCDIR)/toolkit/mozapps/update/updater/dep1.der)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/dep2Cert.h.stub
$(TOPOBJDIR)/toolkit/mozapps/update/updater/dep2Cert.h: $(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/dep2Cert.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/dep2Cert.h.pp
$(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/dep2Cert.h.stub: /worker/build/toolkit/mozapps/update/updater/gen_cert_header.py $(TOPSRCDIR)/toolkit/mozapps/update/updater/dep2.der
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/update/updater/gen_cert_header.py create_header $(TOPOBJDIR)/toolkit/mozapps/update/updater/dep2Cert.h $(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/dep2Cert.h.pp $(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/dep2Cert.h.stub $(TOPSRCDIR)/toolkit/mozapps/update/updater/dep2.der)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/primaryCert.h.stub
$(TOPOBJDIR)/toolkit/mozapps/update/updater/primaryCert.h: $(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/primaryCert.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/primaryCert.h.pp
$(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/primaryCert.h.stub: /worker/build/toolkit/mozapps/update/updater/gen_cert_header.py $(TOPSRCDIR)/toolkit/mozapps/update/updater/release_primary.der
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/update/updater/gen_cert_header.py create_header $(TOPOBJDIR)/toolkit/mozapps/update/updater/primaryCert.h $(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/primaryCert.h.pp $(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/primaryCert.h.stub $(TOPSRCDIR)/toolkit/mozapps/update/updater/release_primary.der)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/secondaryCert.h.stub
$(TOPOBJDIR)/toolkit/mozapps/update/updater/secondaryCert.h: $(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/secondaryCert.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/secondaryCert.h.pp
$(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/secondaryCert.h.stub: /worker/build/toolkit/mozapps/update/updater/gen_cert_header.py $(TOPSRCDIR)/toolkit/mozapps/update/updater/release_secondary.der
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/update/updater/gen_cert_header.py create_header $(TOPOBJDIR)/toolkit/mozapps/update/updater/secondaryCert.h $(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/secondaryCert.h.pp $(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/secondaryCert.h.stub $(TOPSRCDIR)/toolkit/mozapps/update/updater/release_secondary.der)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/xpcshellCert.h.stub
$(TOPOBJDIR)/toolkit/mozapps/update/updater/xpcshellCert.h: $(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/xpcshellCert.h.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/xpcshellCert.h.pp
$(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/xpcshellCert.h.stub: /worker/build/toolkit/mozapps/update/updater/gen_cert_header.py $(TOPSRCDIR)/toolkit/mozapps/update/updater/xpcshellCertificate.der
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/update/updater/gen_cert_header.py create_header $(TOPOBJDIR)/toolkit/mozapps/update/updater/xpcshellCert.h $(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/xpcshellCert.h.pp $(TOPOBJDIR)/toolkit/mozapps/update/updater/$(MDDEPDIR)/xpcshellCert.h.stub $(TOPSRCDIR)/toolkit/mozapps/update/updater/xpcshellCertificate.der)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/main.js: $(TOPOBJDIR)/devtools/client/debugger/src/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/vendors.js: $(TOPOBJDIR)/devtools/client/debugger/src/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/main.js $(TOPSRCDIR)/devtools/client/debugger/src/vendors.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/main.js $(TOPSRCDIR)/devtools/client/debugger/src/vendors.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/actions/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/event-listeners.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/exceptions.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/expressions.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/file-search.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/index.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/navigation.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/preview.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/project-text-search.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/quick-open.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/source-actors.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/source-tree.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/tabs.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/toolbox.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/threads.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/ui.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/event-listeners.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/exceptions.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/expressions.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/file-search.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/index.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/navigation.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/preview.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/project-text-search.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/quick-open.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/source-actors.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/source-tree.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/tabs.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/toolbox.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/threads.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/ui.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/actions/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/event-listeners.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/exceptions.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/expressions.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/file-search.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/index.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/navigation.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/preview.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/project-text-search.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/quick-open.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/source-actors.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/source-tree.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/tabs.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/toolbox.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/threads.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/ui.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/actions/ast/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/actions/ast/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/actions/ast/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/ast/index.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/ast/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/ast/setInScopeLines.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/ast/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/actions/ast/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/actions/ast/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/ast/index.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/ast/setInScopeLines.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/actions/ast/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/actions/ast/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/actions/ast/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/ast/index.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/ast/setInScopeLines.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/ast)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/actions/breakpoints/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/actions/breakpoints/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/actions/breakpoints/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/breakpoints/breakpointPositions.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/breakpoints/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/breakpoints/index.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/breakpoints/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/breakpoints/modify.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/breakpoints/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/breakpoints/remapLocations.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/breakpoints/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/breakpoints/syncBreakpoint.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/breakpoints/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/actions/breakpoints/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/actions/breakpoints/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/breakpoints/breakpointPositions.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/breakpoints/index.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/breakpoints/modify.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/breakpoints/remapLocations.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/breakpoints/syncBreakpoint.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/actions/breakpoints/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/actions/breakpoints/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/actions/breakpoints/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/breakpoints/breakpointPositions.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/breakpoints/index.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/breakpoints/modify.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/breakpoints/remapLocations.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/breakpoints/syncBreakpoint.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/breakpoints)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/actions/pause/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/breakOnNext.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/commands.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/continueToHere.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/expandScopes.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/fetchFrames.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/fetchScopes.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/index.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/inlinePreview.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/mapDisplayNames.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/mapFrames.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/mapScopes.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/paused.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/pauseOnExceptions.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/previewPausedLocation.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/resumed.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/selectFrame.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/highlightCalls.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/skipPausing.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/breakOnNext.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/commands.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/continueToHere.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/expandScopes.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/fetchFrames.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/fetchScopes.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/index.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/inlinePreview.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/mapDisplayNames.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/mapFrames.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/mapScopes.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/paused.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/pauseOnExceptions.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/previewPausedLocation.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/resumed.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/selectFrame.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/highlightCalls.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/skipPausing.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/actions/pause/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/breakOnNext.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/commands.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/continueToHere.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/expandScopes.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/fetchFrames.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/fetchScopes.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/index.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/inlinePreview.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/mapDisplayNames.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/mapFrames.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/mapScopes.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/paused.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/pauseOnExceptions.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/previewPausedLocation.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/resumed.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/selectFrame.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/highlightCalls.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/pause/skipPausing.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/actions/sources/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/sources/blackbox.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/sources/breakableLines.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/sources/index.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/sources/loadSourceText.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/sources/newSources.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/sources/prettyPrint.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/sources/select.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/sources/symbols.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/sources/blackbox.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/sources/breakableLines.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/sources/index.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/sources/loadSourceText.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/sources/newSources.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/sources/prettyPrint.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/sources/select.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/sources/symbols.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/actions/sources/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/sources/blackbox.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/sources/breakableLines.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/sources/index.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/sources/loadSourceText.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/sources/newSources.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/sources/prettyPrint.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/sources/select.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/sources/symbols.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/sources)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/actions/utils/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/actions/utils/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/actions/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/utils/create-store.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/utils/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/actions/utils/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/actions/utils/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/utils/create-store.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/actions/utils/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/actions/utils/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/actions/utils/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/utils/create-store.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/utils)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/actions/utils/middleware/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/actions/utils/middleware/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/actions/utils/middleware/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/utils/middleware/context.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/utils/middleware/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/utils/middleware/log.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/utils/middleware/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/utils/middleware/promise.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/utils/middleware/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/utils/middleware/thunk.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/utils/middleware/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/utils/middleware/timing.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/utils/middleware/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/utils/middleware/wait-service.js: $(TOPOBJDIR)/devtools/client/debugger/src/actions/utils/middleware/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/actions/utils/middleware/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/actions/utils/middleware/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/utils/middleware/context.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/utils/middleware/log.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/utils/middleware/promise.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/utils/middleware/thunk.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/utils/middleware/timing.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/utils/middleware/wait-service.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/actions/utils/middleware/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/actions/utils/middleware/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/actions/utils/middleware/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/utils/middleware/context.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/utils/middleware/log.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/utils/middleware/promise.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/utils/middleware/thunk.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/utils/middleware/timing.js $(TOPSRCDIR)/devtools/client/debugger/src/actions/utils/middleware/wait-service.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/utils/middleware)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/client/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/client/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/client/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/client/firefox.js: $(TOPOBJDIR)/devtools/client/debugger/src/client/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/client/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/client/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/client/firefox.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/client/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/client/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/client/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/client/firefox.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/client)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/client/firefox/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/client/firefox/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/client/firefox/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/client/firefox/commands.js: $(TOPOBJDIR)/devtools/client/debugger/src/client/firefox/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/client/firefox/create.js: $(TOPOBJDIR)/devtools/client/debugger/src/client/firefox/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/client/firefox/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/client/firefox/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/client/firefox/commands.js $(TOPSRCDIR)/devtools/client/debugger/src/client/firefox/create.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/client/firefox/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/client/firefox/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/client/firefox/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/client/firefox/commands.js $(TOPSRCDIR)/devtools/client/debugger/src/client/firefox/create.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/client/firefox)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/components/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/components/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/components/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/A11yIntention.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/App.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/ProjectSearch.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/QuickOpenModal.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/ShortcutsModal.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/WelcomeBox.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/components/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/components/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/components/A11yIntention.js $(TOPSRCDIR)/devtools/client/debugger/src/components/App.js $(TOPSRCDIR)/devtools/client/debugger/src/components/ProjectSearch.js $(TOPSRCDIR)/devtools/client/debugger/src/components/QuickOpenModal.js $(TOPSRCDIR)/devtools/client/debugger/src/components/ShortcutsModal.js $(TOPSRCDIR)/devtools/client/debugger/src/components/WelcomeBox.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/components/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/components/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/components/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/components/A11yIntention.js $(TOPSRCDIR)/devtools/client/debugger/src/components/App.js $(TOPSRCDIR)/devtools/client/debugger/src/components/ProjectSearch.js $(TOPSRCDIR)/devtools/client/debugger/src/components/QuickOpenModal.js $(TOPSRCDIR)/devtools/client/debugger/src/components/ShortcutsModal.js $(TOPSRCDIR)/devtools/client/debugger/src/components/WelcomeBox.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/Breakpoint.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/Breakpoints.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/ColumnBreakpoint.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/ColumnBreakpoints.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/ConditionalPanel.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/DebugLine.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/EditorMenu.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/EmptyLines.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/Exception.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/Exceptions.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/Footer.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/HighlightCalls.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/HighlightLine.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/HighlightLines.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/index.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/InlinePreview.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/InlinePreviewRow.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/InlinePreviews.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/SearchBar.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/Tab.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/Tabs.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/Breakpoint.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/Breakpoints.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/ColumnBreakpoint.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/ColumnBreakpoints.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/ConditionalPanel.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/DebugLine.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/EditorMenu.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/EmptyLines.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/Exception.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/Exceptions.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/Footer.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/HighlightCalls.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/HighlightLine.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/HighlightLines.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/index.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/InlinePreview.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/InlinePreviewRow.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/InlinePreviews.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/SearchBar.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/Tab.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/Tabs.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/Breakpoint.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/Breakpoints.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/ColumnBreakpoint.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/ColumnBreakpoints.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/ConditionalPanel.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/DebugLine.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/EditorMenu.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/EmptyLines.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/Exception.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/Exceptions.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/Footer.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/HighlightCalls.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/HighlightLine.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/HighlightLines.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/index.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/InlinePreview.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/InlinePreviewRow.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/InlinePreviews.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/SearchBar.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/Tab.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/Tabs.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/menus/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/menus/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/menus/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/menus/breakpoints.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/menus/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/menus/editor.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/menus/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/menus/source.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/menus/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/menus/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/menus/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/menus/breakpoints.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/menus/editor.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/menus/source.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/menus/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/menus/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/menus/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/menus/breakpoints.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/menus/editor.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/menus/source.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/menus)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/Preview/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/Preview/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/Preview/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/Preview/ExceptionPopup.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/Preview/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/Preview/index.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/Preview/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/Preview/Popup.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/Preview/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/Preview/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/Preview/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/Preview/ExceptionPopup.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/Preview/index.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/Preview/Popup.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/Preview/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/Preview/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/components/Editor/Preview/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/Preview/ExceptionPopup.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/Preview/index.js $(TOPSRCDIR)/devtools/client/debugger/src/components/Editor/Preview/Popup.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/Preview)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/components/PrimaryPanes/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/components/PrimaryPanes/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/components/PrimaryPanes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/PrimaryPanes/index.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/PrimaryPanes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/PrimaryPanes/Outline.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/PrimaryPanes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/PrimaryPanes/OutlineFilter.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/PrimaryPanes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/PrimaryPanes/SourcesTree.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/PrimaryPanes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/PrimaryPanes/SourcesTreeItem.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/PrimaryPanes/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/components/PrimaryPanes/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/components/PrimaryPanes/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/components/PrimaryPanes/index.js $(TOPSRCDIR)/devtools/client/debugger/src/components/PrimaryPanes/Outline.js $(TOPSRCDIR)/devtools/client/debugger/src/components/PrimaryPanes/OutlineFilter.js $(TOPSRCDIR)/devtools/client/debugger/src/components/PrimaryPanes/SourcesTree.js $(TOPSRCDIR)/devtools/client/debugger/src/components/PrimaryPanes/SourcesTreeItem.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/components/PrimaryPanes/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/components/PrimaryPanes/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/components/PrimaryPanes/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/components/PrimaryPanes/index.js $(TOPSRCDIR)/devtools/client/debugger/src/components/PrimaryPanes/Outline.js $(TOPSRCDIR)/devtools/client/debugger/src/components/PrimaryPanes/OutlineFilter.js $(TOPSRCDIR)/devtools/client/debugger/src/components/PrimaryPanes/SourcesTree.js $(TOPSRCDIR)/devtools/client/debugger/src/components/PrimaryPanes/SourcesTreeItem.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/PrimaryPanes)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/CommandBar.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/DOMMutationBreakpoints.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/EventListeners.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Expressions.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/index.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Scopes.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Thread.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Threads.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/WhyPaused.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/XHRBreakpoints.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/CommandBar.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/DOMMutationBreakpoints.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/EventListeners.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Expressions.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/index.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Scopes.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Thread.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Threads.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/WhyPaused.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/XHRBreakpoints.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/CommandBar.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/DOMMutationBreakpoints.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/EventListeners.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Expressions.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/index.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Scopes.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Thread.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Threads.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/WhyPaused.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/XHRBreakpoints.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/Breakpoint.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/BreakpointHeading.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/BreakpointHeadingsContextMenu.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/BreakpointsContextMenu.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/ExceptionOption.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/index.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/Breakpoint.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/BreakpointHeading.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/BreakpointHeadingsContextMenu.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/BreakpointsContextMenu.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/ExceptionOption.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/index.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/Breakpoint.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/BreakpointHeading.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/BreakpointHeadingsContextMenu.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/BreakpointsContextMenu.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/ExceptionOption.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/index.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/Frames/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/Frames/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/Frames/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Frames/Frame.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/Frames/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Frames/FrameIndent.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/Frames/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Frames/FrameMenu.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/Frames/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Frames/Group.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/Frames/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Frames/index.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/Frames/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/Frames/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/Frames/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Frames/Frame.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Frames/FrameIndent.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Frames/FrameMenu.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Frames/Group.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Frames/index.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/Frames/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/Frames/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/components/SecondaryPanes/Frames/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Frames/Frame.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Frames/FrameIndent.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Frames/FrameMenu.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Frames/Group.js $(TOPSRCDIR)/devtools/client/debugger/src/components/SecondaryPanes/Frames/index.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Frames)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/components/shared/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/AccessibleImage.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/Accordion.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/Badge.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/BracketArrow.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/Dropdown.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/ManagedTree.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/Modal.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/Popover.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/PreviewFunction.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/ResultList.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/SearchInput.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/SourceIcon.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/SmartGap.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/AccessibleImage.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/Accordion.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/Badge.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/BracketArrow.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/Dropdown.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/ManagedTree.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/Modal.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/Popover.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/PreviewFunction.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/ResultList.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/SearchInput.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/SourceIcon.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/SmartGap.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/AccessibleImage.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/Accordion.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/Badge.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/BracketArrow.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/Dropdown.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/ManagedTree.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/Modal.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/Popover.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/PreviewFunction.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/ResultList.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/SearchInput.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/SourceIcon.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/SmartGap.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/Button/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/components/shared/Button/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/Button/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/Button/CloseButton.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/Button/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/Button/CommandBarButton.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/Button/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/Button/index.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/Button/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/Button/PaneToggleButton.js: $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/Button/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/Button/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/components/shared/Button/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/Button/CloseButton.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/Button/CommandBarButton.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/Button/index.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/Button/PaneToggleButton.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/Button/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/Button/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/Button/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/Button/CloseButton.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/Button/CommandBarButton.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/Button/index.js $(TOPSRCDIR)/devtools/client/debugger/src/components/shared/Button/PaneToggleButton.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/Button)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/Button/styles/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/components/shared/Button/styles/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/Button/styles/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/Button/styles/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/components/shared/Button/styles/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/Button/styles/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/Button/styles/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/components/shared/Button/styles/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/Button/styles)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/context-menu/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/context-menu/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/context-menu/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/context-menu/menu.js: $(TOPOBJDIR)/devtools/client/debugger/src/context-menu/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/context-menu/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/context-menu/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/context-menu/menu.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/context-menu/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/context-menu/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/context-menu/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/context-menu/menu.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/context-menu)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/reducers/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/ast.js: $(TOPOBJDIR)/devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/async-requests.js: $(TOPOBJDIR)/devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/breakpoints.js: $(TOPOBJDIR)/devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/event-listeners.js: $(TOPOBJDIR)/devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/exceptions.js: $(TOPOBJDIR)/devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/expressions.js: $(TOPOBJDIR)/devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/file-search.js: $(TOPOBJDIR)/devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/index.js: $(TOPOBJDIR)/devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/pause.js: $(TOPOBJDIR)/devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/pending-breakpoints.js: $(TOPOBJDIR)/devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/preview.js: $(TOPOBJDIR)/devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/project-text-search.js: $(TOPOBJDIR)/devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/quick-open.js: $(TOPOBJDIR)/devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/source-actors.js: $(TOPOBJDIR)/devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/source-tree.js: $(TOPOBJDIR)/devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/sources.js: $(TOPOBJDIR)/devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/tabs.js: $(TOPOBJDIR)/devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/threads.js: $(TOPOBJDIR)/devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/ui.js: $(TOPOBJDIR)/devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/ast.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/async-requests.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/breakpoints.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/event-listeners.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/exceptions.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/expressions.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/file-search.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/index.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/pause.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/pending-breakpoints.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/preview.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/project-text-search.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/quick-open.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/source-actors.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/source-tree.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/sources.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/tabs.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/threads.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/ui.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/reducers/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/ast.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/async-requests.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/breakpoints.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/event-listeners.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/exceptions.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/expressions.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/file-search.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/index.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/pause.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/pending-breakpoints.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/preview.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/project-text-search.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/quick-open.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/source-actors.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/source-tree.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/sources.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/tabs.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/threads.js $(TOPSRCDIR)/devtools/client/debugger/src/reducers/ui.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/selectors/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/selectors/breakpointAtLocation.js: $(TOPOBJDIR)/devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/selectors/breakpoints.js: $(TOPOBJDIR)/devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/selectors/breakpointSources.js: $(TOPOBJDIR)/devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/selectors/getCallStackFrames.js: $(TOPOBJDIR)/devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/selectors/inComponent.js: $(TOPOBJDIR)/devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/selectors/index.js: $(TOPOBJDIR)/devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/selectors/isLineInScope.js: $(TOPOBJDIR)/devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/selectors/isSelectedFrameVisible.js: $(TOPOBJDIR)/devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/selectors/pause.js: $(TOPOBJDIR)/devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/selectors/visibleBreakpoints.js: $(TOPOBJDIR)/devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/selectors/visibleColumnBreakpoints.js: $(TOPOBJDIR)/devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/selectors/breakpointAtLocation.js $(TOPSRCDIR)/devtools/client/debugger/src/selectors/breakpoints.js $(TOPSRCDIR)/devtools/client/debugger/src/selectors/breakpointSources.js $(TOPSRCDIR)/devtools/client/debugger/src/selectors/getCallStackFrames.js $(TOPSRCDIR)/devtools/client/debugger/src/selectors/inComponent.js $(TOPSRCDIR)/devtools/client/debugger/src/selectors/index.js $(TOPSRCDIR)/devtools/client/debugger/src/selectors/isLineInScope.js $(TOPSRCDIR)/devtools/client/debugger/src/selectors/isSelectedFrameVisible.js $(TOPSRCDIR)/devtools/client/debugger/src/selectors/pause.js $(TOPSRCDIR)/devtools/client/debugger/src/selectors/visibleBreakpoints.js $(TOPSRCDIR)/devtools/client/debugger/src/selectors/visibleColumnBreakpoints.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/selectors/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/selectors/breakpointAtLocation.js $(TOPSRCDIR)/devtools/client/debugger/src/selectors/breakpoints.js $(TOPSRCDIR)/devtools/client/debugger/src/selectors/breakpointSources.js $(TOPSRCDIR)/devtools/client/debugger/src/selectors/getCallStackFrames.js $(TOPSRCDIR)/devtools/client/debugger/src/selectors/inComponent.js $(TOPSRCDIR)/devtools/client/debugger/src/selectors/index.js $(TOPSRCDIR)/devtools/client/debugger/src/selectors/isLineInScope.js $(TOPSRCDIR)/devtools/client/debugger/src/selectors/isSelectedFrameVisible.js $(TOPSRCDIR)/devtools/client/debugger/src/selectors/pause.js $(TOPSRCDIR)/devtools/client/debugger/src/selectors/visibleBreakpoints.js $(TOPSRCDIR)/devtools/client/debugger/src/selectors/visibleColumnBreakpoints.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/selectors)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/utils/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/assert.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/ast.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/async-value.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/bootstrap.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/build-query.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/clipboard.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/connect.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/context.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/dbg.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/defer.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/DevToolsUtils.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/environment.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/expressions.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/evaluation-result.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/function.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/indentation.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/isMinified.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/location.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/log.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/memoize.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/memoizeLast.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/memoizableAction.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/path.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/prefs.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/preview.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/project-search.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/quick-open.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/result-list.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/selected-location.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/source-maps.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/source-queue.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/source.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/tabs.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/task.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/telemetry.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/text.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/timings.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/ui.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/url.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/utils.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/wasm.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/worker.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/assert.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/ast.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/async-value.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/bootstrap.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/build-query.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/clipboard.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/connect.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/context.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/dbg.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/defer.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/DevToolsUtils.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/environment.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/expressions.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/evaluation-result.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/function.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/indentation.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/isMinified.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/location.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/log.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/memoize.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/memoizeLast.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/memoizableAction.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/path.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/prefs.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/preview.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/project-search.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/quick-open.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/result-list.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/selected-location.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/source-maps.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/source-queue.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/source.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/tabs.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/task.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/telemetry.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/text.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/timings.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/ui.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/url.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/utils.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/wasm.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/worker.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/utils/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/assert.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/ast.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/async-value.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/bootstrap.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/build-query.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/clipboard.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/connect.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/context.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/dbg.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/defer.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/DevToolsUtils.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/environment.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/expressions.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/evaluation-result.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/function.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/indentation.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/isMinified.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/location.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/log.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/memoize.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/memoizeLast.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/memoizableAction.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/path.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/prefs.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/preview.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/project-search.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/quick-open.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/result-list.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/selected-location.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/source-maps.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/source-queue.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/source.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/tabs.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/task.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/telemetry.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/text.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/timings.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/ui.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/url.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/utils.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/wasm.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/worker.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/utils/breakpoint/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/utils/breakpoint/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/utils/breakpoint/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/breakpoint/astBreakpointLocation.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/breakpoint/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/breakpoint/breakpointPositions.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/breakpoint/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/breakpoint/index.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/breakpoint/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/utils/breakpoint/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/utils/breakpoint/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/breakpoint/astBreakpointLocation.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/breakpoint/breakpointPositions.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/breakpoint/index.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/utils/breakpoint/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/utils/breakpoint/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/utils/breakpoint/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/breakpoint/astBreakpointLocation.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/breakpoint/breakpointPositions.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/breakpoint/index.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/breakpoint)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/utils/editor/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/editor/create-editor.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/editor/get-expression.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/editor/get-token-location.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/editor/index.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/editor/source-documents.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/editor/source-editor.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/editor/source-search.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/editor/token-events.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/editor/create-editor.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/editor/get-expression.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/editor/get-token-location.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/editor/index.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/editor/source-documents.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/editor/source-editor.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/editor/source-search.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/editor/token-events.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/utils/editor/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/editor/create-editor.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/editor/get-expression.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/editor/get-token-location.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/editor/index.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/editor/source-documents.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/editor/source-editor.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/editor/source-search.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/editor/token-events.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/editor)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/index.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/why.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/index.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/why.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/index.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/why.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/frames/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/frames/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/frames/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/frames/annotateFrames.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/frames/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/frames/collapseFrames.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/frames/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/frames/displayName.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/frames/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/frames/getFrameUrl.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/frames/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/frames/getLibraryFromUrl.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/frames/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/frames/index.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/frames/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/frames/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/frames/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/frames/annotateFrames.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/frames/collapseFrames.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/frames/displayName.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/frames/getFrameUrl.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/frames/getLibraryFromUrl.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/frames/index.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/frames/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/frames/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/frames/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/frames/annotateFrames.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/frames/collapseFrames.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/frames/displayName.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/frames/getFrameUrl.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/frames/getLibraryFromUrl.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/frames/index.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/frames)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/mapScopes/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/mapScopes/buildGeneratedBindingList.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/mapScopes/filtering.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/mapScopes/findGeneratedBindingFromPosition.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/mapScopes/getApplicableBindingsForOriginalPosition.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/mapScopes/index.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/mapScopes/locColumn.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/mapScopes/mappingContains.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/mapScopes/optimizedOut.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/mapScopes/positionCmp.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/mapScopes/rangeMetadata.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/mapScopes/buildGeneratedBindingList.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/mapScopes/filtering.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/mapScopes/findGeneratedBindingFromPosition.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/mapScopes/getApplicableBindingsForOriginalPosition.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/mapScopes/index.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/mapScopes/locColumn.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/mapScopes/mappingContains.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/mapScopes/optimizedOut.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/mapScopes/positionCmp.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/mapScopes/rangeMetadata.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/mapScopes/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/mapScopes/buildGeneratedBindingList.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/mapScopes/filtering.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/mapScopes/findGeneratedBindingFromPosition.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/mapScopes/getApplicableBindingsForOriginalPosition.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/mapScopes/index.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/mapScopes/locColumn.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/mapScopes/mappingContains.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/mapScopes/optimizedOut.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/mapScopes/positionCmp.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/mapScopes/rangeMetadata.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/mapScopes)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/scopes/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/scopes/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/scopes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/scopes/getScope.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/scopes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/scopes/getVariables.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/scopes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/scopes/index.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/scopes/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/scopes/utils.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/scopes/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/scopes/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/scopes/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/scopes/getScope.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/scopes/getVariables.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/scopes/index.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/scopes/utils.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/scopes/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/scopes/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/utils/pause/scopes/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/scopes/getScope.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/scopes/getVariables.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/scopes/index.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/pause/scopes/utils.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/scopes)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/utils/resource/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/resource/base-query.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/resource/compare.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/resource/core.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/resource/index.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/resource/memoize.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/resource/query-cache.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/resource/query.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/resource/selector.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/resource/base-query.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/resource/compare.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/resource/core.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/resource/index.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/resource/memoize.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/resource/query-cache.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/resource/query.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/resource/selector.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/utils/resource/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/resource/base-query.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/resource/compare.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/resource/core.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/resource/index.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/resource/memoize.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/resource/query-cache.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/resource/query.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/resource/selector.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/resource)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/utils/sources-tree/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/sources-tree/addToTree.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/sources-tree/collapseTree.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/sources-tree/formatTree.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/sources-tree/getDirectories.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/sources-tree/getURL.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/sources-tree/index.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/sources-tree/sortTree.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/sources-tree/treeOrder.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/sources-tree/updateTree.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/sources-tree/utils.js: $(TOPOBJDIR)/devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/sources-tree/addToTree.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/sources-tree/collapseTree.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/sources-tree/formatTree.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/sources-tree/getDirectories.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/sources-tree/getURL.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/sources-tree/index.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/sources-tree/sortTree.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/sources-tree/treeOrder.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/sources-tree/updateTree.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/sources-tree/utils.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/utils/sources-tree/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/sources-tree/addToTree.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/sources-tree/collapseTree.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/sources-tree/formatTree.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/sources-tree/getDirectories.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/sources-tree/getURL.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/sources-tree/index.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/sources-tree/sortTree.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/sources-tree/treeOrder.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/sources-tree/updateTree.js $(TOPSRCDIR)/devtools/client/debugger/src/utils/sources-tree/utils.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/sources-tree)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/workers/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/workers/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/workers/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/workers/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/workers/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/workers/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/workers/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/workers/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/workers)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/workers/parser/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/workers/parser/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/workers/parser/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/workers/parser/index.js: $(TOPOBJDIR)/devtools/client/debugger/src/workers/parser/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/workers/parser/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/workers/parser/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/workers/parser/index.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/workers/parser/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/workers/parser/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/workers/parser/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/workers/parser/index.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/workers/parser)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/workers/pretty-print/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/workers/pretty-print/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/workers/pretty-print/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/workers/pretty-print/index.js: $(TOPOBJDIR)/devtools/client/debugger/src/workers/pretty-print/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/workers/pretty-print/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/workers/pretty-print/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/workers/pretty-print/index.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/workers/pretty-print/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/workers/pretty-print/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/workers/pretty-print/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/workers/pretty-print/index.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/workers/pretty-print)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/client/debugger/src/workers/search/$(MDDEPDIR)/node.stub.stub
$(TOPOBJDIR)/devtools/client/debugger/src/workers/search/node.stub: $(TOPOBJDIR)/devtools/client/debugger/src/workers/search/$(MDDEPDIR)/node.stub.stub ;
$(TOPOBJDIR)/worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/workers/search/index.js: $(TOPOBJDIR)/devtools/client/debugger/src/workers/search/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/client/debugger/src/workers/search/$(MDDEPDIR)/node.stub.pp
$(TOPOBJDIR)/devtools/client/debugger/src/workers/search/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/workers/search/index.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate $(TOPOBJDIR)/devtools/client/debugger/src/workers/search/node.stub $(TOPOBJDIR)/devtools/client/debugger/src/workers/search/$(MDDEPDIR)/node.stub.pp $(TOPOBJDIR)/devtools/client/debugger/src/workers/search/$(MDDEPDIR)/node.stub.stub $(TOPSRCDIR)/devtools/client/shared/build/build.js $(TOPSRCDIR)/devtools/client/debugger/src/workers/search/index.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/workers/search)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/devtools/shared/webconsole/$(MDDEPDIR)/reserved-js-words.js.stub
$(TOPOBJDIR)/devtools/shared/webconsole/reserved-js-words.js: $(TOPOBJDIR)/devtools/shared/webconsole/$(MDDEPDIR)/reserved-js-words.js.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/devtools/shared/webconsole/$(MDDEPDIR)/reserved-js-words.js.pp
$(TOPOBJDIR)/devtools/shared/webconsole/$(MDDEPDIR)/reserved-js-words.js.stub: /worker/build/devtools/shared/webconsole/GenerateReservedWordsJS.py $(TOPSRCDIR)/js/src/frontend/ReservedWords.h
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/devtools/shared/webconsole/GenerateReservedWordsJS.py main $(TOPOBJDIR)/devtools/shared/webconsole/reserved-js-words.js $(TOPOBJDIR)/devtools/shared/webconsole/$(MDDEPDIR)/reserved-js-words.js.pp $(TOPOBJDIR)/devtools/shared/webconsole/$(MDDEPDIR)/reserved-js-words.js.stub $(TOPSRCDIR)/js/src/frontend/ReservedWords.h)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/library/$(MDDEPDIR)/buildid.cpp.stub
$(TOPOBJDIR)/toolkit/library/buildid.cpp: $(TOPOBJDIR)/toolkit/library/$(MDDEPDIR)/buildid.cpp.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/library/$(MDDEPDIR)/buildid.cpp.pp
$(TOPOBJDIR)/toolkit/library/$(MDDEPDIR)/buildid.cpp.stub: /worker/build/toolkit/library/gen_buildid.py $(TOPOBJDIR)/toolkit/library/build/libxul_so.list
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/library/gen_buildid.py main $(TOPOBJDIR)/toolkit/library/buildid.cpp $(TOPOBJDIR)/toolkit/library/$(MDDEPDIR)/buildid.cpp.pp $(TOPOBJDIR)/toolkit/library/$(MDDEPDIR)/buildid.cpp.stub $(TOPOBJDIR)/toolkit/library/build/libxul_so.list)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/library/build/$(MDDEPDIR)/symverscript.stub
$(TOPOBJDIR)/toolkit/library/build/symverscript: $(TOPOBJDIR)/toolkit/library/build/$(MDDEPDIR)/symverscript.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/library/build/$(MDDEPDIR)/symverscript.pp
$(TOPOBJDIR)/toolkit/library/build/$(MDDEPDIR)/symverscript.stub: /worker/build/build/gen_symverscript.py $(TOPSRCDIR)/toolkit/library/symverscript.in
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/gen_symverscript.py main $(TOPOBJDIR)/toolkit/library/build/symverscript $(TOPOBJDIR)/toolkit/library/build/$(MDDEPDIR)/symverscript.pp $(TOPOBJDIR)/toolkit/library/build/$(MDDEPDIR)/symverscript.stub $(TOPSRCDIR)/toolkit/library/symverscript.in xul87)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/library/build/$(MDDEPDIR)/dependentlibs.list.stub
$(TOPOBJDIR)/toolkit/library/build/dependentlibs.list: $(TOPOBJDIR)/toolkit/library/build/$(MDDEPDIR)/dependentlibs.list.stub ;
$(TOPOBJDIR)/toolkit/library/build/dependentlibs.list.gtest: $(TOPOBJDIR)/toolkit/library/build/$(MDDEPDIR)/dependentlibs.list.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/library/build/$(MDDEPDIR)/dependentlibs.list.pp
$(TOPOBJDIR)/toolkit/library/build/$(MDDEPDIR)/dependentlibs.list.stub: /worker/build/toolkit/library/build/dependentlibs.py $(TOPOBJDIR)/toolkit/library/build/libxul.so
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/library/build/dependentlibs.py gen_list $(TOPOBJDIR)/toolkit/library/build/dependentlibs.list $(TOPOBJDIR)/toolkit/library/build/$(MDDEPDIR)/dependentlibs.list.pp $(TOPOBJDIR)/toolkit/library/build/$(MDDEPDIR)/dependentlibs.list.stub $(TOPOBJDIR)/toolkit/library/build/libxul.so)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/toolkit/library/gtest/$(MDDEPDIR)/symverscript.stub
$(TOPOBJDIR)/toolkit/library/gtest/symverscript: $(TOPOBJDIR)/toolkit/library/gtest/$(MDDEPDIR)/symverscript.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/toolkit/library/gtest/$(MDDEPDIR)/symverscript.pp
$(TOPOBJDIR)/toolkit/library/gtest/$(MDDEPDIR)/symverscript.stub: /worker/build/build/gen_symverscript.py $(TOPSRCDIR)/toolkit/library/symverscript.in
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/gen_symverscript.py main $(TOPOBJDIR)/toolkit/library/gtest/symverscript $(TOPOBJDIR)/toolkit/library/gtest/$(MDDEPDIR)/symverscript.pp $(TOPOBJDIR)/toolkit/library/gtest/$(MDDEPDIR)/symverscript.stub $(TOPSRCDIR)/toolkit/library/symverscript.in xul87)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_root.pem.stub
$(TOPOBJDIR)/services/settings/test/unit/test_remote_settings_signatures/collection_signing_root.pem: $(TOPOBJDIR)/services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_root.pem.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_root.pem.pp
$(TOPOBJDIR)/services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_root.pem.stub: /worker/build/security/manager/ssl/tests/unit/pycert.py $(TOPSRCDIR)/services/settings/test/unit/test_remote_settings_signatures/collection_signing_root.pem.certspec
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/manager/ssl/tests/unit/pycert.py main $(TOPOBJDIR)/services/settings/test/unit/test_remote_settings_signatures/collection_signing_root.pem $(TOPOBJDIR)/services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_root.pem.pp $(TOPOBJDIR)/services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_root.pem.stub $(TOPSRCDIR)/services/settings/test/unit/test_remote_settings_signatures/collection_signing_root.pem.certspec)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_int.pem.stub
$(TOPOBJDIR)/services/settings/test/unit/test_remote_settings_signatures/collection_signing_int.pem: $(TOPOBJDIR)/services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_int.pem.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_int.pem.pp
$(TOPOBJDIR)/services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_int.pem.stub: /worker/build/security/manager/ssl/tests/unit/pycert.py $(TOPSRCDIR)/services/settings/test/unit/test_remote_settings_signatures/collection_signing_int.pem.certspec
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/manager/ssl/tests/unit/pycert.py main $(TOPOBJDIR)/services/settings/test/unit/test_remote_settings_signatures/collection_signing_int.pem $(TOPOBJDIR)/services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_int.pem.pp $(TOPOBJDIR)/services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_int.pem.stub $(TOPSRCDIR)/services/settings/test/unit/test_remote_settings_signatures/collection_signing_int.pem.certspec)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_ee.pem.stub
$(TOPOBJDIR)/services/settings/test/unit/test_remote_settings_signatures/collection_signing_ee.pem: $(TOPOBJDIR)/services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_ee.pem.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_ee.pem.pp
$(TOPOBJDIR)/services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_ee.pem.stub: /worker/build/security/manager/ssl/tests/unit/pycert.py $(TOPSRCDIR)/services/settings/test/unit/test_remote_settings_signatures/collection_signing_ee.pem.certspec
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/manager/ssl/tests/unit/pycert.py main $(TOPOBJDIR)/services/settings/test/unit/test_remote_settings_signatures/collection_signing_ee.pem $(TOPOBJDIR)/services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_ee.pem.pp $(TOPOBJDIR)/services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_ee.pem.stub $(TOPSRCDIR)/services/settings/test/unit/test_remote_settings_signatures/collection_signing_ee.pem.certspec)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/browser/base/content/$(MDDEPDIR)/aboutNetErrorCodes.js.stub
$(TOPOBJDIR)/browser/base/content/aboutNetErrorCodes.js: $(TOPOBJDIR)/browser/base/content/$(MDDEPDIR)/aboutNetErrorCodes.js.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/browser/base/content/$(MDDEPDIR)/aboutNetErrorCodes.js.pp
$(TOPOBJDIR)/browser/base/content/$(MDDEPDIR)/aboutNetErrorCodes.js.stub: /worker/build/browser/base/gen_aboutneterror_codes.py $(TOPSRCDIR)/browser/locales/en-US/browser/nsserrors.ftl
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/browser/base/gen_aboutneterror_codes.py main $(TOPOBJDIR)/browser/base/content/aboutNetErrorCodes.js $(TOPOBJDIR)/browser/base/content/$(MDDEPDIR)/aboutNetErrorCodes.js.pp $(TOPOBJDIR)/browser/base/content/$(MDDEPDIR)/aboutNetErrorCodes.js.stub $(TOPSRCDIR)/browser/locales/en-US/browser/nsserrors.ftl)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/browser/locales/$(MDDEPDIR)/updater.ini.stub
$(TOPOBJDIR)/browser/locales/updater.ini: $(TOPOBJDIR)/browser/locales/$(MDDEPDIR)/updater.ini.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/browser/locales/$(MDDEPDIR)/updater.ini.pp
$(TOPOBJDIR)/browser/locales/$(MDDEPDIR)/updater.ini.stub: /worker/build/browser/locales/generate_ini.py $(TOPSRCDIR)/browser/locales/en-US/updater/updater.ini $(TOPSRCDIR)/browser/installer/windows/nsis/updater_append.ini $(if $(IS_LANGUAGE_REPACK),FORCE)
	$(REPORT_BUILD)
	$(call py_action,file_generate,--locale=$(AB_CD) /worker/build/browser/locales/generate_ini.py main $(TOPOBJDIR)/browser/locales/updater.ini $(TOPOBJDIR)/browser/locales/$(MDDEPDIR)/updater.ini.pp $(TOPOBJDIR)/browser/locales/$(MDDEPDIR)/updater.ini.stub $(TOPSRCDIR)/browser/locales/en-US/updater/updater.ini $(TOPSRCDIR)/browser/installer/windows/nsis/updater_append.ini)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/browser/locales/$(MDDEPDIR)/bookmarks.html.stub
$(TOPOBJDIR)/browser/locales/bookmarks.html: $(TOPOBJDIR)/browser/locales/$(MDDEPDIR)/bookmarks.html.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/browser/locales/$(MDDEPDIR)/bookmarks.html.pp
$(TOPOBJDIR)/browser/locales/$(MDDEPDIR)/bookmarks.html.stub: /worker/build/browser/locales/generate_bookmarks.py $(TOPSRCDIR)/browser/locales/generic/profile/bookmarks.html.in $(TOPSRCDIR)/browser/locales/en-US/profile/bookmarks.inc $(if $(IS_LANGUAGE_REPACK),FORCE)
	$(REPORT_BUILD)
	$(call py_action,file_generate,--locale=$(AB_CD) /worker/build/browser/locales/generate_bookmarks.py main $(TOPOBJDIR)/browser/locales/bookmarks.html $(TOPOBJDIR)/browser/locales/$(MDDEPDIR)/bookmarks.html.pp $(TOPOBJDIR)/browser/locales/$(MDDEPDIR)/bookmarks.html.stub $(TOPSRCDIR)/browser/locales/generic/profile/bookmarks.html.in $(TOPSRCDIR)/browser/locales/en-US/profile/bookmarks.inc)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/browser/app/macbuild/Contents/$(MDDEPDIR)/MacOS-files.txt.stub
$(TOPOBJDIR)/browser/app/macbuild/Contents/MacOS-files.txt: $(TOPOBJDIR)/browser/app/macbuild/Contents/$(MDDEPDIR)/MacOS-files.txt.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/browser/app/macbuild/Contents/$(MDDEPDIR)/MacOS-files.txt.pp
$(TOPOBJDIR)/browser/app/macbuild/Contents/$(MDDEPDIR)/MacOS-files.txt.stub: /worker/build/python/mozbuild/mozbuild/action/preprocessor.py $(TOPSRCDIR)/browser/app/macbuild/Contents/MacOS-files.in
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/preprocessor.py generate $(TOPOBJDIR)/browser/app/macbuild/Contents/MacOS-files.txt $(TOPOBJDIR)/browser/app/macbuild/Contents/$(MDDEPDIR)/MacOS-files.txt.pp $(TOPOBJDIR)/browser/app/macbuild/Contents/$(MDDEPDIR)/MacOS-files.txt.stub $(TOPSRCDIR)/browser/app/macbuild/Contents/MacOS-files.in -DMOZ_CRASHREPORTER=1 -DMOZ_APP_NAME=konke)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/nss/lib/ckfw/builtins/builtins_nssckbi/$(MDDEPDIR)/certdata.c.stub
$(TOPOBJDIR)/security/nss/lib/ckfw/builtins/builtins_nssckbi/certdata.c: $(TOPOBJDIR)/security/nss/lib/ckfw/builtins/builtins_nssckbi/$(MDDEPDIR)/certdata.c.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/nss/lib/ckfw/builtins/builtins_nssckbi/$(MDDEPDIR)/certdata.c.pp
$(TOPOBJDIR)/security/nss/lib/ckfw/builtins/builtins_nssckbi/$(MDDEPDIR)/certdata.c.stub: /worker/build/security/generate_certdata.py $(TOPSRCDIR)/security/nss/lib/ckfw/builtins/certdata.py $(TOPSRCDIR)/security/nss/lib/ckfw/builtins/certdata.perl $(TOPSRCDIR)/security/nss/lib/ckfw/builtins/certdata.txt
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/generate_certdata.py main $(TOPOBJDIR)/security/nss/lib/ckfw/builtins/builtins_nssckbi/certdata.c $(TOPOBJDIR)/security/nss/lib/ckfw/builtins/builtins_nssckbi/$(MDDEPDIR)/certdata.c.pp $(TOPOBJDIR)/security/nss/lib/ckfw/builtins/builtins_nssckbi/$(MDDEPDIR)/certdata.c.stub $(TOPSRCDIR)/security/nss/lib/ckfw/builtins/certdata.py $(TOPSRCDIR)/security/nss/lib/ckfw/builtins/certdata.perl $(TOPSRCDIR)/security/nss/lib/ckfw/builtins/certdata.txt)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/nss/lib/ckfw/builtins/builtins_nssckbi/$(MDDEPDIR)/out.nssckbi.def.stub
$(TOPOBJDIR)/security/nss/lib/ckfw/builtins/builtins_nssckbi/out.nssckbi.def: $(TOPOBJDIR)/security/nss/lib/ckfw/builtins/builtins_nssckbi/$(MDDEPDIR)/out.nssckbi.def.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/nss/lib/ckfw/builtins/builtins_nssckbi/$(MDDEPDIR)/out.nssckbi.def.pp
$(TOPOBJDIR)/security/nss/lib/ckfw/builtins/builtins_nssckbi/$(MDDEPDIR)/out.nssckbi.def.stub: /worker/build/security/generate_mapfile.py $(TOPSRCDIR)/security/nss/lib/ckfw/builtins/nssckbi.def
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/generate_mapfile.py main $(TOPOBJDIR)/security/nss/lib/ckfw/builtins/builtins_nssckbi/out.nssckbi.def $(TOPOBJDIR)/security/nss/lib/ckfw/builtins/builtins_nssckbi/$(MDDEPDIR)/out.nssckbi.def.pp $(TOPOBJDIR)/security/nss/lib/ckfw/builtins/builtins_nssckbi/$(MDDEPDIR)/out.nssckbi.def.stub $(TOPSRCDIR)/security/nss/lib/ckfw/builtins/nssckbi.def)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/nss/lib/freebl/freebl_freebl3/$(MDDEPDIR)/out.freebl_hash.def.stub
$(TOPOBJDIR)/security/nss/lib/freebl/freebl_freebl3/out.freebl_hash.def: $(TOPOBJDIR)/security/nss/lib/freebl/freebl_freebl3/$(MDDEPDIR)/out.freebl_hash.def.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/nss/lib/freebl/freebl_freebl3/$(MDDEPDIR)/out.freebl_hash.def.pp
$(TOPOBJDIR)/security/nss/lib/freebl/freebl_freebl3/$(MDDEPDIR)/out.freebl_hash.def.stub: /worker/build/security/generate_mapfile.py $(TOPSRCDIR)/security/nss/lib/freebl/freebl_hash.def
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/generate_mapfile.py main $(TOPOBJDIR)/security/nss/lib/freebl/freebl_freebl3/out.freebl_hash.def $(TOPOBJDIR)/security/nss/lib/freebl/freebl_freebl3/$(MDDEPDIR)/out.freebl_hash.def.pp $(TOPOBJDIR)/security/nss/lib/freebl/freebl_freebl3/$(MDDEPDIR)/out.freebl_hash.def.stub $(TOPSRCDIR)/security/nss/lib/freebl/freebl_hash.def)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/nss/lib/freebl/freebl_freeblpriv3/$(MDDEPDIR)/out.freebl_hash_vector.def.stub
$(TOPOBJDIR)/security/nss/lib/freebl/freebl_freeblpriv3/out.freebl_hash_vector.def: $(TOPOBJDIR)/security/nss/lib/freebl/freebl_freeblpriv3/$(MDDEPDIR)/out.freebl_hash_vector.def.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/nss/lib/freebl/freebl_freeblpriv3/$(MDDEPDIR)/out.freebl_hash_vector.def.pp
$(TOPOBJDIR)/security/nss/lib/freebl/freebl_freeblpriv3/$(MDDEPDIR)/out.freebl_hash_vector.def.stub: /worker/build/security/generate_mapfile.py $(TOPSRCDIR)/security/nss/lib/freebl/freebl_hash_vector.def
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/generate_mapfile.py main $(TOPOBJDIR)/security/nss/lib/freebl/freebl_freeblpriv3/out.freebl_hash_vector.def $(TOPOBJDIR)/security/nss/lib/freebl/freebl_freeblpriv3/$(MDDEPDIR)/out.freebl_hash_vector.def.pp $(TOPOBJDIR)/security/nss/lib/freebl/freebl_freeblpriv3/$(MDDEPDIR)/out.freebl_hash_vector.def.stub $(TOPSRCDIR)/security/nss/lib/freebl/freebl_hash_vector.def)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/nss/lib/nss/nss_nss3/$(MDDEPDIR)/out.nss.def.stub
$(TOPOBJDIR)/security/nss/lib/nss/nss_nss3/out.nss.def: $(TOPOBJDIR)/security/nss/lib/nss/nss_nss3/$(MDDEPDIR)/out.nss.def.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/nss/lib/nss/nss_nss3/$(MDDEPDIR)/out.nss.def.pp
$(TOPOBJDIR)/security/nss/lib/nss/nss_nss3/$(MDDEPDIR)/out.nss.def.stub: /worker/build/security/generate_mapfile.py $(TOPSRCDIR)/security/nss/lib/nss/nss.def
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/generate_mapfile.py main $(TOPOBJDIR)/security/nss/lib/nss/nss_nss3/out.nss.def $(TOPOBJDIR)/security/nss/lib/nss/nss_nss3/$(MDDEPDIR)/out.nss.def.pp $(TOPOBJDIR)/security/nss/lib/nss/nss_nss3/$(MDDEPDIR)/out.nss.def.stub $(TOPSRCDIR)/security/nss/lib/nss/nss.def)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/nss/lib/smime/smime_smime3/$(MDDEPDIR)/out.smime.def.stub
$(TOPOBJDIR)/security/nss/lib/smime/smime_smime3/out.smime.def: $(TOPOBJDIR)/security/nss/lib/smime/smime_smime3/$(MDDEPDIR)/out.smime.def.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/nss/lib/smime/smime_smime3/$(MDDEPDIR)/out.smime.def.pp
$(TOPOBJDIR)/security/nss/lib/smime/smime_smime3/$(MDDEPDIR)/out.smime.def.stub: /worker/build/security/generate_mapfile.py $(TOPSRCDIR)/security/nss/lib/smime/smime.def
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/generate_mapfile.py main $(TOPOBJDIR)/security/nss/lib/smime/smime_smime3/out.smime.def $(TOPOBJDIR)/security/nss/lib/smime/smime_smime3/$(MDDEPDIR)/out.smime.def.pp $(TOPOBJDIR)/security/nss/lib/smime/smime_smime3/$(MDDEPDIR)/out.smime.def.stub $(TOPSRCDIR)/security/nss/lib/smime/smime.def)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/nss/lib/softoken/softoken_softokn3/$(MDDEPDIR)/out.softokn.def.stub
$(TOPOBJDIR)/security/nss/lib/softoken/softoken_softokn3/out.softokn.def: $(TOPOBJDIR)/security/nss/lib/softoken/softoken_softokn3/$(MDDEPDIR)/out.softokn.def.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/nss/lib/softoken/softoken_softokn3/$(MDDEPDIR)/out.softokn.def.pp
$(TOPOBJDIR)/security/nss/lib/softoken/softoken_softokn3/$(MDDEPDIR)/out.softokn.def.stub: /worker/build/security/generate_mapfile.py $(TOPSRCDIR)/security/nss/lib/softoken/softokn.def
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/generate_mapfile.py main $(TOPOBJDIR)/security/nss/lib/softoken/softoken_softokn3/out.softokn.def $(TOPOBJDIR)/security/nss/lib/softoken/softoken_softokn3/$(MDDEPDIR)/out.softokn.def.pp $(TOPOBJDIR)/security/nss/lib/softoken/softoken_softokn3/$(MDDEPDIR)/out.softokn.def.stub $(TOPSRCDIR)/security/nss/lib/softoken/softokn.def)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/nss/lib/ssl/ssl_ssl3/$(MDDEPDIR)/out.ssl.def.stub
$(TOPOBJDIR)/security/nss/lib/ssl/ssl_ssl3/out.ssl.def: $(TOPOBJDIR)/security/nss/lib/ssl/ssl_ssl3/$(MDDEPDIR)/out.ssl.def.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/nss/lib/ssl/ssl_ssl3/$(MDDEPDIR)/out.ssl.def.pp
$(TOPOBJDIR)/security/nss/lib/ssl/ssl_ssl3/$(MDDEPDIR)/out.ssl.def.stub: /worker/build/security/generate_mapfile.py $(TOPSRCDIR)/security/nss/lib/ssl/ssl.def
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/generate_mapfile.py main $(TOPOBJDIR)/security/nss/lib/ssl/ssl_ssl3/out.ssl.def $(TOPOBJDIR)/security/nss/lib/ssl/ssl_ssl3/$(MDDEPDIR)/out.ssl.def.pp $(TOPOBJDIR)/security/nss/lib/ssl/ssl_ssl3/$(MDDEPDIR)/out.ssl.def.stub $(TOPSRCDIR)/security/nss/lib/ssl/ssl.def)
	@$(TOUCH) $@

default: $(TOPOBJDIR)/security/nss/lib/util/util_nssutil3/$(MDDEPDIR)/out.nssutil.def.stub
$(TOPOBJDIR)/security/nss/lib/util/util_nssutil3/out.nssutil.def: $(TOPOBJDIR)/security/nss/lib/util/util_nssutil3/$(MDDEPDIR)/out.nssutil.def.stub ;
EXTRA_MDDEPEND_FILES += $(TOPOBJDIR)/security/nss/lib/util/util_nssutil3/$(MDDEPDIR)/out.nssutil.def.pp
$(TOPOBJDIR)/security/nss/lib/util/util_nssutil3/$(MDDEPDIR)/out.nssutil.def.stub: /worker/build/security/generate_mapfile.py $(TOPSRCDIR)/security/nss/lib/util/nssutil.def
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/generate_mapfile.py main $(TOPOBJDIR)/security/nss/lib/util/util_nssutil3/out.nssutil.def $(TOPOBJDIR)/security/nss/lib/util/util_nssutil3/$(MDDEPDIR)/out.nssutil.def.pp $(TOPOBJDIR)/security/nss/lib/util/util_nssutil3/$(MDDEPDIR)/out.nssutil.def.stub $(TOPSRCDIR)/security/nss/lib/util/nssutil.def)
	@$(TOUCH) $@

