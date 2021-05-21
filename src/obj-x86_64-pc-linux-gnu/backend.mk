# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1 -Dtop_srcdir=/worker/build
DIRS := build config python testing third_party/python taskcluster js/src config/external/fdlibm config/external/nspr config/external/zlib memory mfbt mozglue config/external/icu config/external/ffi tools xpcom/xpidl config/external security toolkit/library/gtest/rust toolkit/library/rust security/sandbox security/certverifier security/apps modules/libmar config/external/freetype2 xpcom modules/libpref intl netwerk extensions/auth toolkit/mozapps/update/updater/bspatch other-licenses/bsdiff ipc hal js/xpconnect modules/libjar storage media/libyuv extensions/permissions media/webrtc dom/media/webrtc/transport testing/specialpowers testing/gtest uriloader caps parser gfx image dom view widget editor layout docshell xpfe/appshell extensions/universalchardet accessible tools/code-coverage tools/performance tools/power tools/profiler extensions/spellcheck security/manager toolkit extensions/pref devtools toolkit/library services startupcache js/ductwork/debugger other-licenses/snappy toolkit/system/gnome remote testing/firefox-ui testing/marionette toolkit/components/telemetry/tests/marionette testing/geckodriver tools/quitter media/gmp-clearkey/0.1 testing/mochitest testing/xpcshell testing/tools/minidumpwriter testing/tools/screenshot testing/profiles testing/modules testing/runtimes testing/web-platform testing/extensions memory/gtest dom/media/webrtc/transport/test browser/branding/dot browser
include $(topsrcdir)/config/AB_rCD.mk
EXPORT_TARGETS += $(MDDEPDIR)/mozilla-config.h.stub
mozilla-config.h: $(MDDEPDIR)/mozilla-config.h.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/mozilla-config.h.pp
$(MDDEPDIR)/mozilla-config.h.stub: /worker/build/python/mozbuild/mozbuild/action/process_define_files.py $(srcdir)/mozilla-config.h.in
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/process_define_files.py process_define_file mozilla-config.h $(MDDEPDIR)/mozilla-config.h.pp $(MDDEPDIR)/mozilla-config.h.stub $(srcdir)/mozilla-config.h.in)
	@$(TOUCH) $@

EXPORT_TARGETS += $(MDDEPDIR)/buildid.h.stub
buildid.h: $(MDDEPDIR)/buildid.h.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/buildid.h.pp
$(MDDEPDIR)/buildid.h.stub: /worker/build/build/variables.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/variables.py buildid_header buildid.h $(MDDEPDIR)/buildid.h.pp $(MDDEPDIR)/buildid.h.stub)
	@$(TOUCH) $@

EXPORT_TARGETS += $(MDDEPDIR)/source-repo.h.stub
source-repo.h: $(MDDEPDIR)/source-repo.h.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/source-repo.h.pp
$(MDDEPDIR)/source-repo.h.stub: /worker/build/build/variables.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/variables.py source_repo_header source-repo.h $(MDDEPDIR)/source-repo.h.pp $(MDDEPDIR)/source-repo.h.stub)
	@$(TOUCH) $@

dist_include_FILES += buildid.h
dist_include_FILES += mozilla-config.h
dist_include_FILES += source-repo.h
dist_include_DEST := $(DEPTH)/dist/include/
dist_include_TARGET := export
INSTALL_TARGETS += dist_include
OBJDIR_PP_FILES_0 += $(srcdir)/.cargo/config.in
OBJDIR_PP_FILES_0_PATH := $(DEPTH)/.cargo
OBJDIR_PP_FILES_0_TARGET := misc
PP_TARGETS += OBJDIR_PP_FILES_0
build/misc: build/$(MDDEPDIR)/application.ini.stub
build/application.ini: build/$(MDDEPDIR)/application.ini.stub ;
EXTRA_MDDEPEND_FILES += build/$(MDDEPDIR)/application.ini.pp
build/$(MDDEPDIR)/application.ini.stub: /worker/build/python/mozbuild/mozbuild/action/preprocessor.py $(srcdir)/build/application.ini.in backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/preprocessor.py generate build/application.ini build/$(MDDEPDIR)/application.ini.pp build/$(MDDEPDIR)/application.ini.stub $(srcdir)/build/application.ini.in -DGRE_MILESTONE=87.0 -DMAR_CHANNEL_ID= -DMOZ_APPUPDATE_HOST=push.dothq.co -DMOZ_APP_BASENAME=Konke '-DMOZ_APP_DISPLAYNAME=Konke Browser' '-DMOZ_APP_ID={ec8030f7-c20a-464f-9b0e-13a3a9e97384}' -DMOZ_APP_PROFILE=konke -DMOZ_APP_REMOTINGNAME=konke '-DMOZ_APP_VENDOR=Konke HQ' -DMOZ_APP_VERSION=87.0 -DMOZ_BUILD_APP_IS_BROWSER=1 -DMOZ_CRASHREPORTER=1 -DMOZ_PROFILE_MIGRATOR=1 -DMOZ_UPDATER=1 -DTOPOBJDIR=/worker/build/obj-x86_64-pc-linux-gnu)
	@$(TOUCH) $@

build/export: build/$(MDDEPDIR)/application.ini.h.stub
build/application.ini.h: build/$(MDDEPDIR)/application.ini.h.stub ;
EXTRA_MDDEPEND_FILES += build/$(MDDEPDIR)/application.ini.h.pp
build/$(MDDEPDIR)/application.ini.h.stub: /worker/build/build/appini_header.py build/application.ini
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/appini_header.py main build/application.ini.h build/$(MDDEPDIR)/application.ini.h.pp build/$(MDDEPDIR)/application.ini.h.stub build/application.ini)
	@$(TOUCH) $@

config/export: config/$(MDDEPDIR)/stl.sentinel.stub
config/stl.sentinel: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/new: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/algorithm: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/atomic: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/cassert: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/climits: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/cmath: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/condition_variable: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/cstdarg: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/cstdio: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/cstdlib: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/cstring: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/cwchar: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/deque: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/functional: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/ios: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/iosfwd: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/iostream: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/istream: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/iterator: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/limits: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/list: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/map: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/memory: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/mutex: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/ostream: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/regex: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/set: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/shared_mutex: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/stack: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/string: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/thread: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/tuple: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/type_traits: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/unordered_map: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/unordered_set: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/utility: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/vector: config/$(MDDEPDIR)/stl.sentinel.stub ;
dist/stl_wrappers/xutility: config/$(MDDEPDIR)/stl.sentinel.stub ;
EXTRA_MDDEPEND_FILES += config/$(MDDEPDIR)/stl.sentinel.pp
config/$(MDDEPDIR)/stl.sentinel.stub: /worker/build/config/make-stl-wrappers.py backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/config/make-stl-wrappers.py gen_wrappers config/stl.sentinel config/$(MDDEPDIR)/stl.sentinel.pp config/$(MDDEPDIR)/stl.sentinel.stub /worker/build/obj-x86_64-pc-linux-gnu/dist/stl_wrappers gcc /worker/build/config/gcc-stl-wrapper.template.h new algorithm atomic cassert climits cmath condition_variable cstdarg cstdio cstdlib cstring cwchar deque functional ios iosfwd iostream istream iterator limits list map memory mutex ostream regex set shared_mutex stack string thread tuple type_traits unordered_map unordered_set utility vector xutility)
	@$(TOUCH) $@

config/misc: config/$(MDDEPDIR)/cbindgen-metadata.json.stub
config/cbindgen-metadata.json: config/$(MDDEPDIR)/cbindgen-metadata.json.stub ;
EXTRA_MDDEPEND_FILES += config/$(MDDEPDIR)/cbindgen-metadata.json.pp
config/$(MDDEPDIR)/cbindgen-metadata.json.stub: /worker/build/build/RunCbindgen.py .cargo/config
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate_metadata config/cbindgen-metadata.json config/$(MDDEPDIR)/cbindgen-metadata.json.pp config/$(MDDEPDIR)/cbindgen-metadata.json.stub .cargo/config)
	@$(TOUCH) $@

js/src/export: js/src/$(MDDEPDIR)/js-confdefs.h.stub
js/src/js-confdefs.h: js/src/$(MDDEPDIR)/js-confdefs.h.stub ;
EXTRA_MDDEPEND_FILES += js/src/$(MDDEPDIR)/js-confdefs.h.pp
js/src/$(MDDEPDIR)/js-confdefs.h.stub: /worker/build/python/mozbuild/mozbuild/action/process_define_files.py $(srcdir)/js/src/js-confdefs.h.in
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/process_define_files.py process_define_file js/src/js-confdefs.h js/src/$(MDDEPDIR)/js-confdefs.h.pp js/src/$(MDDEPDIR)/js-confdefs.h.stub $(srcdir)/js/src/js-confdefs.h.in)
	@$(TOUCH) $@

js/src/export: js/src/$(MDDEPDIR)/js-config.h.stub
js/src/js-config.h: js/src/$(MDDEPDIR)/js-config.h.stub ;
EXTRA_MDDEPEND_FILES += js/src/$(MDDEPDIR)/js-config.h.pp
js/src/$(MDDEPDIR)/js-config.h.stub: /worker/build/python/mozbuild/mozbuild/action/process_define_files.py $(srcdir)/js/src/js-config.h.in
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/process_define_files.py process_define_file js/src/js-config.h js/src/$(MDDEPDIR)/js-config.h.pp js/src/$(MDDEPDIR)/js-config.h.stub $(srcdir)/js/src/js-config.h.in)
	@$(TOUCH) $@

js/src/export: js/src/$(MDDEPDIR)/selfhosted.out.h.stub
js/src/selfhosted.out.h: js/src/$(MDDEPDIR)/selfhosted.out.h.stub ;
js/src/selfhosted.js: js/src/$(MDDEPDIR)/selfhosted.out.h.stub ;
EXTRA_MDDEPEND_FILES += js/src/$(MDDEPDIR)/selfhosted.out.h.pp
js/src/$(MDDEPDIR)/selfhosted.out.h.stub: /worker/build/js/src/builtin/embedjs.py $(srcdir)/js/public/friend/ErrorNumbers.msg $(srcdir)/js/src/builtin/TypedArrayConstants.h $(srcdir)/js/src/builtin/SelfHostingDefines.h $(srcdir)/js/src/builtin/Utilities.js $(srcdir)/js/src/builtin/Array.js $(srcdir)/js/src/builtin/AsyncFunction.js $(srcdir)/js/src/builtin/AsyncIteration.js $(srcdir)/js/src/builtin/BigInt.js $(srcdir)/js/src/builtin/Date.js $(srcdir)/js/src/builtin/Error.js $(srcdir)/js/src/builtin/Function.js $(srcdir)/js/src/builtin/Generator.js $(srcdir)/js/src/builtin/Iterator.js $(srcdir)/js/src/builtin/Map.js $(srcdir)/js/src/builtin/Module.js $(srcdir)/js/src/builtin/Number.js $(srcdir)/js/src/builtin/Object.js $(srcdir)/js/src/builtin/Promise.js $(srcdir)/js/src/builtin/Reflect.js $(srcdir)/js/src/builtin/RegExp.js $(srcdir)/js/src/builtin/RegExpGlobalReplaceOpt.h.js $(srcdir)/js/src/builtin/RegExpLocalReplaceOpt.h.js $(srcdir)/js/src/builtin/String.js $(srcdir)/js/src/builtin/Set.js $(srcdir)/js/src/builtin/Sorting.js $(srcdir)/js/src/builtin/TypedArray.js $(srcdir)/js/src/builtin/WeakMap.js $(srcdir)/js/src/builtin/WeakSet.js $(srcdir)/js/src/builtin/intl/NumberingSystemsGenerated.h $(srcdir)/js/src/builtin/intl/Collator.js $(srcdir)/js/src/builtin/intl/CommonFunctions.js $(srcdir)/js/src/builtin/intl/CurrencyDataGenerated.js $(srcdir)/js/src/builtin/intl/DateTimeFormat.js $(srcdir)/js/src/builtin/intl/DisplayNames.js $(srcdir)/js/src/builtin/intl/IntlObject.js $(srcdir)/js/src/builtin/intl/ListFormat.js $(srcdir)/js/src/builtin/intl/NumberFormat.js $(srcdir)/js/src/builtin/intl/PluralRules.js $(srcdir)/js/src/builtin/intl/RelativeTimeFormat.js $(srcdir)/js/src/builtin/intl/SanctionedSimpleUnitIdentifiersGenerated.js
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/js/src/builtin/embedjs.py generate_selfhosted js/src/selfhosted.out.h js/src/$(MDDEPDIR)/selfhosted.out.h.pp js/src/$(MDDEPDIR)/selfhosted.out.h.stub $(srcdir)/js/public/friend/ErrorNumbers.msg $(srcdir)/js/src/builtin/TypedArrayConstants.h $(srcdir)/js/src/builtin/SelfHostingDefines.h $(srcdir)/js/src/builtin/Utilities.js $(srcdir)/js/src/builtin/Array.js $(srcdir)/js/src/builtin/AsyncFunction.js $(srcdir)/js/src/builtin/AsyncIteration.js $(srcdir)/js/src/builtin/BigInt.js $(srcdir)/js/src/builtin/Date.js $(srcdir)/js/src/builtin/Error.js $(srcdir)/js/src/builtin/Function.js $(srcdir)/js/src/builtin/Generator.js $(srcdir)/js/src/builtin/Iterator.js $(srcdir)/js/src/builtin/Map.js $(srcdir)/js/src/builtin/Module.js $(srcdir)/js/src/builtin/Number.js $(srcdir)/js/src/builtin/Object.js $(srcdir)/js/src/builtin/Promise.js $(srcdir)/js/src/builtin/Reflect.js $(srcdir)/js/src/builtin/RegExp.js $(srcdir)/js/src/builtin/RegExpGlobalReplaceOpt.h.js $(srcdir)/js/src/builtin/RegExpLocalReplaceOpt.h.js $(srcdir)/js/src/builtin/String.js $(srcdir)/js/src/builtin/Set.js $(srcdir)/js/src/builtin/Sorting.js $(srcdir)/js/src/builtin/TypedArray.js $(srcdir)/js/src/builtin/WeakMap.js $(srcdir)/js/src/builtin/WeakSet.js $(srcdir)/js/src/builtin/intl/NumberingSystemsGenerated.h $(srcdir)/js/src/builtin/intl/Collator.js $(srcdir)/js/src/builtin/intl/CommonFunctions.js $(srcdir)/js/src/builtin/intl/CurrencyDataGenerated.js $(srcdir)/js/src/builtin/intl/DateTimeFormat.js $(srcdir)/js/src/builtin/intl/DisplayNames.js $(srcdir)/js/src/builtin/intl/IntlObject.js $(srcdir)/js/src/builtin/intl/ListFormat.js $(srcdir)/js/src/builtin/intl/NumberFormat.js $(srcdir)/js/src/builtin/intl/PluralRules.js $(srcdir)/js/src/builtin/intl/RelativeTimeFormat.js $(srcdir)/js/src/builtin/intl/SanctionedSimpleUnitIdentifiersGenerated.js)
	@$(TOUCH) $@

js/src/build/misc: js/src/build/$(MDDEPDIR)/spidermonkey_checks.stub
js/src/build/spidermonkey_checks: js/src/build/$(MDDEPDIR)/spidermonkey_checks.stub ;
EXTRA_MDDEPEND_FILES += js/src/build/$(MDDEPDIR)/spidermonkey_checks.pp
js/src/build/$(MDDEPDIR)/spidermonkey_checks.stub: /worker/build/config/run_spidermonkey_checks.py js/src/build/libjs_static.a $(srcdir)/config/check_spidermonkey_style.py $(srcdir)/config/check_macroassembler_style.py $(srcdir)/config/check_js_opcode.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/config/run_spidermonkey_checks.py main js/src/build/spidermonkey_checks js/src/build/$(MDDEPDIR)/spidermonkey_checks.pp js/src/build/$(MDDEPDIR)/spidermonkey_checks.stub js/src/build/libjs_static.a $(srcdir)/config/check_spidermonkey_style.py $(srcdir)/config/check_macroassembler_style.py $(srcdir)/config/check_js_opcode.py)
	@$(TOUCH) $@

js/src/frontend/export: js/src/frontend/$(MDDEPDIR)/ReservedWordsGenerated.h.stub
js/src/frontend/ReservedWordsGenerated.h: js/src/frontend/$(MDDEPDIR)/ReservedWordsGenerated.h.stub ;
EXTRA_MDDEPEND_FILES += js/src/frontend/$(MDDEPDIR)/ReservedWordsGenerated.h.pp
js/src/frontend/$(MDDEPDIR)/ReservedWordsGenerated.h.stub: /worker/build/js/src/frontend/GenerateReservedWords.py $(srcdir)/js/src/frontend/ReservedWords.h
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/js/src/frontend/GenerateReservedWords.py main js/src/frontend/ReservedWordsGenerated.h js/src/frontend/$(MDDEPDIR)/ReservedWordsGenerated.h.pp js/src/frontend/$(MDDEPDIR)/ReservedWordsGenerated.h.stub $(srcdir)/js/src/frontend/ReservedWords.h)
	@$(TOUCH) $@

js/src/gc/export: js/src/gc/$(MDDEPDIR)/StatsPhasesGenerated.h.stub
js/src/gc/StatsPhasesGenerated.h: js/src/gc/$(MDDEPDIR)/StatsPhasesGenerated.h.stub ;
EXTRA_MDDEPEND_FILES += js/src/gc/$(MDDEPDIR)/StatsPhasesGenerated.h.pp
js/src/gc/$(MDDEPDIR)/StatsPhasesGenerated.h.stub: /worker/build/js/src/gc/GenerateStatsPhases.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/js/src/gc/GenerateStatsPhases.py generateHeader js/src/gc/StatsPhasesGenerated.h js/src/gc/$(MDDEPDIR)/StatsPhasesGenerated.h.pp js/src/gc/$(MDDEPDIR)/StatsPhasesGenerated.h.stub)
	@$(TOUCH) $@

js/src/jit/export: js/src/jit/$(MDDEPDIR)/MOpcodesGenerated.h.stub
js/src/jit/MOpcodesGenerated.h: js/src/jit/$(MDDEPDIR)/MOpcodesGenerated.h.stub ;
EXTRA_MDDEPEND_FILES += js/src/jit/$(MDDEPDIR)/MOpcodesGenerated.h.pp
js/src/jit/$(MDDEPDIR)/MOpcodesGenerated.h.stub: /worker/build/js/src/jit/GenerateOpcodeFiles.py $(srcdir)/js/src/jit/MIR.h
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/js/src/jit/GenerateOpcodeFiles.py generate_mir_header js/src/jit/MOpcodesGenerated.h js/src/jit/$(MDDEPDIR)/MOpcodesGenerated.h.pp js/src/jit/$(MDDEPDIR)/MOpcodesGenerated.h.stub $(srcdir)/js/src/jit/MIR.h)
	@$(TOUCH) $@

js/src/jit/export: js/src/jit/$(MDDEPDIR)/LOpcodesGenerated.h.stub
js/src/jit/LOpcodesGenerated.h: js/src/jit/$(MDDEPDIR)/LOpcodesGenerated.h.stub ;
EXTRA_MDDEPEND_FILES += js/src/jit/$(MDDEPDIR)/LOpcodesGenerated.h.pp
js/src/jit/$(MDDEPDIR)/LOpcodesGenerated.h.stub: /worker/build/js/src/jit/GenerateOpcodeFiles.py $(srcdir)/js/src/jit/LIR.h $(srcdir)/js/src/jit/shared/LIR-shared.h $(srcdir)/js/src/jit/x86-shared/LIR-x86-shared.h $(srcdir)/js/src/jit/x64/LIR-x64.h
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/js/src/jit/GenerateOpcodeFiles.py generate_lir_header js/src/jit/LOpcodesGenerated.h js/src/jit/$(MDDEPDIR)/LOpcodesGenerated.h.pp js/src/jit/$(MDDEPDIR)/LOpcodesGenerated.h.stub $(srcdir)/js/src/jit/LIR.h $(srcdir)/js/src/jit/shared/LIR-shared.h $(srcdir)/js/src/jit/x86-shared/LIR-x86-shared.h $(srcdir)/js/src/jit/x64/LIR-x64.h)
	@$(TOUCH) $@

js/src/jit/export: js/src/jit/$(MDDEPDIR)/CacheIROpsGenerated.h.stub
js/src/jit/CacheIROpsGenerated.h: js/src/jit/$(MDDEPDIR)/CacheIROpsGenerated.h.stub ;
EXTRA_MDDEPEND_FILES += js/src/jit/$(MDDEPDIR)/CacheIROpsGenerated.h.pp
js/src/jit/$(MDDEPDIR)/CacheIROpsGenerated.h.stub: /worker/build/js/src/jit/GenerateCacheIRFiles.py $(srcdir)/js/src/jit/CacheIROps.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/js/src/jit/GenerateCacheIRFiles.py generate_cacheirops_header js/src/jit/CacheIROpsGenerated.h js/src/jit/$(MDDEPDIR)/CacheIROpsGenerated.h.pp js/src/jit/$(MDDEPDIR)/CacheIROpsGenerated.h.stub $(srcdir)/js/src/jit/CacheIROps.yaml)
	@$(TOUCH) $@

config/external/ffi/export: js/src/ctypes/libffi/$(MDDEPDIR)/fficonfig.h.stub
js/src/ctypes/libffi/fficonfig.h: js/src/ctypes/libffi/$(MDDEPDIR)/fficonfig.h.stub ;
EXTRA_MDDEPEND_FILES += js/src/ctypes/libffi/$(MDDEPDIR)/fficonfig.h.pp
js/src/ctypes/libffi/$(MDDEPDIR)/fficonfig.h.stub: /worker/build/python/mozbuild/mozbuild/action/process_define_files.py $(srcdir)/js/src/ctypes/libffi/fficonfig.h.in
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/process_define_files.py process_define_file js/src/ctypes/libffi/fficonfig.h js/src/ctypes/libffi/$(MDDEPDIR)/fficonfig.h.pp js/src/ctypes/libffi/$(MDDEPDIR)/fficonfig.h.stub $(srcdir)/js/src/ctypes/libffi/fficonfig.h.in)
	@$(TOUCH) $@

config/external/ffi/export: js/src/ctypes/libffi/include/$(MDDEPDIR)/ffi.h.stub
js/src/ctypes/libffi/include/ffi.h: js/src/ctypes/libffi/include/$(MDDEPDIR)/ffi.h.stub ;
EXTRA_MDDEPEND_FILES += js/src/ctypes/libffi/include/$(MDDEPDIR)/ffi.h.pp
js/src/ctypes/libffi/include/$(MDDEPDIR)/ffi.h.stub: /worker/build/config/external/ffi/subst_header.py $(srcdir)/js/src/ctypes/libffi/include/ffi.h.in
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/config/external/ffi/subst_header.py main js/src/ctypes/libffi/include/ffi.h js/src/ctypes/libffi/include/$(MDDEPDIR)/ffi.h.pp js/src/ctypes/libffi/include/$(MDDEPDIR)/ffi.h.stub $(srcdir)/js/src/ctypes/libffi/include/ffi.h.in)
	@$(TOUCH) $@

media/mp4parse-rust/export: media/mp4parse-rust/$(MDDEPDIR)/mp4parse_ffi_generated.h.stub
media/mp4parse-rust/mp4parse_ffi_generated.h: media/mp4parse-rust/$(MDDEPDIR)/mp4parse_ffi_generated.h.stub ;
EXTRA_MDDEPEND_FILES += media/mp4parse-rust/$(MDDEPDIR)/mp4parse_ffi_generated.h.pp
media/mp4parse-rust/$(MDDEPDIR)/mp4parse_ffi_generated.h.stub: /worker/build/build/RunCbindgen.py config/cbindgen-metadata.json $(srcdir)/third_party/rust/mp4parse_capi
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate media/mp4parse-rust/mp4parse_ffi_generated.h media/mp4parse-rust/$(MDDEPDIR)/mp4parse_ffi_generated.h.pp media/mp4parse-rust/$(MDDEPDIR)/mp4parse_ffi_generated.h.stub config/cbindgen-metadata.json $(srcdir)/third_party/rust/mp4parse_capi)
	@$(TOUCH) $@

xpcom/export: xpcom/$(MDDEPDIR)/xpcom-config.h.stub
xpcom/xpcom-config.h: xpcom/$(MDDEPDIR)/xpcom-config.h.stub ;
EXTRA_MDDEPEND_FILES += xpcom/$(MDDEPDIR)/xpcom-config.h.pp
xpcom/$(MDDEPDIR)/xpcom-config.h.stub: /worker/build/python/mozbuild/mozbuild/action/process_define_files.py $(srcdir)/xpcom/xpcom-config.h.in
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/process_define_files.py process_define_file xpcom/xpcom-config.h xpcom/$(MDDEPDIR)/xpcom-config.h.pp xpcom/$(MDDEPDIR)/xpcom-config.h.stub $(srcdir)/xpcom/xpcom-config.h.in)
	@$(TOUCH) $@

xpcom/idl-parser/xpidl/export: xpcom/idl-parser/xpidl/$(MDDEPDIR)/xpidl.stub.stub
xpcom/idl-parser/xpidl/xpidl.stub: xpcom/idl-parser/xpidl/$(MDDEPDIR)/xpidl.stub.stub ;
EXTRA_MDDEPEND_FILES += xpcom/idl-parser/xpidl/$(MDDEPDIR)/xpidl.stub.pp
xpcom/idl-parser/xpidl/$(MDDEPDIR)/xpidl.stub.stub: /worker/build/xpcom/idl-parser/xpidl/header.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/xpcom/idl-parser/xpidl/header.py main xpcom/idl-parser/xpidl/xpidl.stub xpcom/idl-parser/xpidl/$(MDDEPDIR)/xpidl.stub.pp xpcom/idl-parser/xpidl/$(MDDEPDIR)/xpidl.stub.stub)
	@$(TOUCH) $@

xpcom/base/export: xpcom/base/$(MDDEPDIR)/ErrorList.h.stub
xpcom/base/ErrorList.h: xpcom/base/$(MDDEPDIR)/ErrorList.h.stub ;
EXTRA_MDDEPEND_FILES += xpcom/base/$(MDDEPDIR)/ErrorList.h.pp
xpcom/base/$(MDDEPDIR)/ErrorList.h.stub: /worker/build/xpcom/base/ErrorList.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/xpcom/base/ErrorList.py error_list_h xpcom/base/ErrorList.h xpcom/base/$(MDDEPDIR)/ErrorList.h.pp xpcom/base/$(MDDEPDIR)/ErrorList.h.stub)
	@$(TOUCH) $@

xpcom/base/export: xpcom/base/$(MDDEPDIR)/ErrorNamesInternal.h.stub
xpcom/base/ErrorNamesInternal.h: xpcom/base/$(MDDEPDIR)/ErrorNamesInternal.h.stub ;
EXTRA_MDDEPEND_FILES += xpcom/base/$(MDDEPDIR)/ErrorNamesInternal.h.pp
xpcom/base/$(MDDEPDIR)/ErrorNamesInternal.h.stub: /worker/build/xpcom/base/ErrorList.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/xpcom/base/ErrorList.py error_names_internal_h xpcom/base/ErrorNamesInternal.h xpcom/base/$(MDDEPDIR)/ErrorNamesInternal.h.pp xpcom/base/$(MDDEPDIR)/ErrorNamesInternal.h.stub)
	@$(TOUCH) $@

xpcom/base/export: xpcom/base/$(MDDEPDIR)/error_list.rs.stub
xpcom/base/error_list.rs: xpcom/base/$(MDDEPDIR)/error_list.rs.stub ;
EXTRA_MDDEPEND_FILES += xpcom/base/$(MDDEPDIR)/error_list.rs.pp
xpcom/base/$(MDDEPDIR)/error_list.rs.stub: /worker/build/xpcom/base/ErrorList.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/xpcom/base/ErrorList.py error_list_rs xpcom/base/error_list.rs xpcom/base/$(MDDEPDIR)/error_list.rs.pp xpcom/base/$(MDDEPDIR)/error_list.rs.stub)
	@$(TOUCH) $@

xpcom/base/export: xpcom/base/$(MDDEPDIR)/gk_rust_utils_ffi_generated.h.stub
xpcom/base/gk_rust_utils_ffi_generated.h: xpcom/base/$(MDDEPDIR)/gk_rust_utils_ffi_generated.h.stub ;
EXTRA_MDDEPEND_FILES += xpcom/base/$(MDDEPDIR)/gk_rust_utils_ffi_generated.h.pp
xpcom/base/$(MDDEPDIR)/gk_rust_utils_ffi_generated.h.stub: /worker/build/build/RunCbindgen.py config/cbindgen-metadata.json $(srcdir)/xpcom/rust/gkrust_utils
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate xpcom/base/gk_rust_utils_ffi_generated.h xpcom/base/$(MDDEPDIR)/gk_rust_utils_ffi_generated.h.pp xpcom/base/$(MDDEPDIR)/gk_rust_utils_ffi_generated.h.stub config/cbindgen-metadata.json $(srcdir)/xpcom/rust/gkrust_utils)
	@$(TOUCH) $@

xpcom/ds/export: xpcom/ds/$(MDDEPDIR)/nsGkAtomList.h.stub
xpcom/ds/nsGkAtomList.h: xpcom/ds/$(MDDEPDIR)/nsGkAtomList.h.stub ;
EXTRA_MDDEPEND_FILES += xpcom/ds/$(MDDEPDIR)/nsGkAtomList.h.pp
xpcom/ds/$(MDDEPDIR)/nsGkAtomList.h.stub: /worker/build/xpcom/ds/StaticAtoms.py $(srcdir)/xpcom/ds/Atom.py $(srcdir)/xpcom/ds/HTMLAtoms.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/xpcom/ds/StaticAtoms.py generate_nsgkatomlist_h xpcom/ds/nsGkAtomList.h xpcom/ds/$(MDDEPDIR)/nsGkAtomList.h.pp xpcom/ds/$(MDDEPDIR)/nsGkAtomList.h.stub $(srcdir)/xpcom/ds/Atom.py $(srcdir)/xpcom/ds/HTMLAtoms.py)
	@$(TOUCH) $@

xpcom/ds/export: xpcom/ds/$(MDDEPDIR)/nsGkAtomConsts.h.stub
xpcom/ds/nsGkAtomConsts.h: xpcom/ds/$(MDDEPDIR)/nsGkAtomConsts.h.stub ;
EXTRA_MDDEPEND_FILES += xpcom/ds/$(MDDEPDIR)/nsGkAtomConsts.h.pp
xpcom/ds/$(MDDEPDIR)/nsGkAtomConsts.h.stub: /worker/build/xpcom/ds/StaticAtoms.py $(srcdir)/xpcom/ds/Atom.py $(srcdir)/xpcom/ds/HTMLAtoms.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/xpcom/ds/StaticAtoms.py generate_nsgkatomconsts_h xpcom/ds/nsGkAtomConsts.h xpcom/ds/$(MDDEPDIR)/nsGkAtomConsts.h.pp xpcom/ds/$(MDDEPDIR)/nsGkAtomConsts.h.stub $(srcdir)/xpcom/ds/Atom.py $(srcdir)/xpcom/ds/HTMLAtoms.py)
	@$(TOUCH) $@

xpcom/build/export: xpcom/build/$(MDDEPDIR)/Services.h.stub
xpcom/build/Services.h: xpcom/build/$(MDDEPDIR)/Services.h.stub ;
EXTRA_MDDEPEND_FILES += xpcom/build/$(MDDEPDIR)/Services.h.pp
xpcom/build/$(MDDEPDIR)/Services.h.stub: /worker/build/xpcom/build/Services.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/xpcom/build/Services.py services_h xpcom/build/Services.h xpcom/build/$(MDDEPDIR)/Services.h.pp xpcom/build/$(MDDEPDIR)/Services.h.stub)
	@$(TOUCH) $@

xpcom/build/export: xpcom/build/$(MDDEPDIR)/services.rs.stub
xpcom/build/services.rs: xpcom/build/$(MDDEPDIR)/services.rs.stub ;
EXTRA_MDDEPEND_FILES += xpcom/build/$(MDDEPDIR)/services.rs.pp
xpcom/build/$(MDDEPDIR)/services.rs.stub: /worker/build/xpcom/build/Services.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/xpcom/build/Services.py services_rs xpcom/build/services.rs xpcom/build/$(MDDEPDIR)/services.rs.pp xpcom/build/$(MDDEPDIR)/services.rs.stub)
	@$(TOUCH) $@

intl/hyphenation/glue/export: intl/hyphenation/glue/$(MDDEPDIR)/mapped_hyph.h.stub
intl/hyphenation/glue/mapped_hyph.h: intl/hyphenation/glue/$(MDDEPDIR)/mapped_hyph.h.stub ;
EXTRA_MDDEPEND_FILES += intl/hyphenation/glue/$(MDDEPDIR)/mapped_hyph.h.pp
intl/hyphenation/glue/$(MDDEPDIR)/mapped_hyph.h.stub: /worker/build/build/RunCbindgen.py config/cbindgen-metadata.json $(srcdir)/third_party/rust/mapped_hyph
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate intl/hyphenation/glue/mapped_hyph.h intl/hyphenation/glue/$(MDDEPDIR)/mapped_hyph.h.pp intl/hyphenation/glue/$(MDDEPDIR)/mapped_hyph.h.stub config/cbindgen-metadata.json $(srcdir)/third_party/rust/mapped_hyph)
	@$(TOUCH) $@

intl/locale/export: intl/locale/$(MDDEPDIR)/encodingsgroups.properties.h.stub
intl/locale/encodingsgroups.properties.h: intl/locale/$(MDDEPDIR)/encodingsgroups.properties.h.stub ;
EXTRA_MDDEPEND_FILES += intl/locale/$(MDDEPDIR)/encodingsgroups.properties.h.pp
intl/locale/$(MDDEPDIR)/encodingsgroups.properties.h.stub: /worker/build/intl/locale/props2arrays.py $(srcdir)/intl/locale/encodingsgroups.properties
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/intl/locale/props2arrays.py main intl/locale/encodingsgroups.properties.h intl/locale/$(MDDEPDIR)/encodingsgroups.properties.h.pp intl/locale/$(MDDEPDIR)/encodingsgroups.properties.h.stub $(srcdir)/intl/locale/encodingsgroups.properties)
	@$(TOUCH) $@

intl/locale/export: intl/locale/$(MDDEPDIR)/fluent_langneg_ffi_generated.h.stub
intl/locale/fluent_langneg_ffi_generated.h: intl/locale/$(MDDEPDIR)/fluent_langneg_ffi_generated.h.stub ;
EXTRA_MDDEPEND_FILES += intl/locale/$(MDDEPDIR)/fluent_langneg_ffi_generated.h.pp
intl/locale/$(MDDEPDIR)/fluent_langneg_ffi_generated.h.stub: /worker/build/build/RunCbindgen.py config/cbindgen-metadata.json $(srcdir)/intl/locale/rust/fluent-langneg-ffi
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate intl/locale/fluent_langneg_ffi_generated.h intl/locale/$(MDDEPDIR)/fluent_langneg_ffi_generated.h.pp intl/locale/$(MDDEPDIR)/fluent_langneg_ffi_generated.h.stub config/cbindgen-metadata.json $(srcdir)/intl/locale/rust/fluent-langneg-ffi)
	@$(TOUCH) $@

intl/locale/export: intl/locale/$(MDDEPDIR)/unic_langid_ffi_generated.h.stub
intl/locale/unic_langid_ffi_generated.h: intl/locale/$(MDDEPDIR)/unic_langid_ffi_generated.h.stub ;
EXTRA_MDDEPEND_FILES += intl/locale/$(MDDEPDIR)/unic_langid_ffi_generated.h.pp
intl/locale/$(MDDEPDIR)/unic_langid_ffi_generated.h.stub: /worker/build/build/RunCbindgen.py config/cbindgen-metadata.json $(srcdir)/intl/locale/rust/unic-langid-ffi
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate intl/locale/unic_langid_ffi_generated.h intl/locale/$(MDDEPDIR)/unic_langid_ffi_generated.h.pp intl/locale/$(MDDEPDIR)/unic_langid_ffi_generated.h.stub config/cbindgen-metadata.json $(srcdir)/intl/locale/rust/unic-langid-ffi)
	@$(TOUCH) $@

intl/l10n/export: intl/l10n/$(MDDEPDIR)/fluent_ffi_generated.h.stub
intl/l10n/fluent_ffi_generated.h: intl/l10n/$(MDDEPDIR)/fluent_ffi_generated.h.stub ;
EXTRA_MDDEPEND_FILES += intl/l10n/$(MDDEPDIR)/fluent_ffi_generated.h.pp
intl/l10n/$(MDDEPDIR)/fluent_ffi_generated.h.stub: /worker/build/build/RunCbindgen.py config/cbindgen-metadata.json $(srcdir)/intl/l10n/rust/fluent-ffi
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate intl/l10n/fluent_ffi_generated.h intl/l10n/$(MDDEPDIR)/fluent_ffi_generated.h.pp intl/l10n/$(MDDEPDIR)/fluent_ffi_generated.h.stub config/cbindgen-metadata.json $(srcdir)/intl/l10n/rust/fluent-ffi)
	@$(TOUCH) $@

netwerk/base/mozurl/export: netwerk/base/mozurl/$(MDDEPDIR)/MozURL_ffi.h.stub
netwerk/base/mozurl/MozURL_ffi.h: netwerk/base/mozurl/$(MDDEPDIR)/MozURL_ffi.h.stub ;
EXTRA_MDDEPEND_FILES += netwerk/base/mozurl/$(MDDEPDIR)/MozURL_ffi.h.pp
netwerk/base/mozurl/$(MDDEPDIR)/MozURL_ffi.h.stub: /worker/build/build/RunCbindgen.py config/cbindgen-metadata.json $(srcdir)/netwerk/base/mozurl
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate netwerk/base/mozurl/MozURL_ffi.h netwerk/base/mozurl/$(MDDEPDIR)/MozURL_ffi.h.pp netwerk/base/mozurl/$(MDDEPDIR)/MozURL_ffi.h.stub config/cbindgen-metadata.json $(srcdir)/netwerk/base/mozurl)
	@$(TOUCH) $@

netwerk/base/rust-helper/export: netwerk/base/rust-helper/$(MDDEPDIR)/rust_helper.h.stub
netwerk/base/rust-helper/rust_helper.h: netwerk/base/rust-helper/$(MDDEPDIR)/rust_helper.h.stub ;
EXTRA_MDDEPEND_FILES += netwerk/base/rust-helper/$(MDDEPDIR)/rust_helper.h.pp
netwerk/base/rust-helper/$(MDDEPDIR)/rust_helper.h.stub: /worker/build/build/RunCbindgen.py config/cbindgen-metadata.json $(srcdir)/netwerk/base/rust-helper
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate netwerk/base/rust-helper/rust_helper.h netwerk/base/rust-helper/$(MDDEPDIR)/rust_helper.h.pp netwerk/base/rust-helper/$(MDDEPDIR)/rust_helper.h.stub config/cbindgen-metadata.json $(srcdir)/netwerk/base/rust-helper)
	@$(TOUCH) $@

netwerk/dns/tests/unit/data/misc: netwerk/dns/tests/unit/data/$(MDDEPDIR)/fake_remote_dafsa.bin.stub
netwerk/dns/tests/unit/data/fake_remote_dafsa.bin: netwerk/dns/tests/unit/data/$(MDDEPDIR)/fake_remote_dafsa.bin.stub ;
EXTRA_MDDEPEND_FILES += netwerk/dns/tests/unit/data/$(MDDEPDIR)/fake_remote_dafsa.bin.pp
netwerk/dns/tests/unit/data/$(MDDEPDIR)/fake_remote_dafsa.bin.stub: /worker/build/netwerk/dns/prepare_tlds.py $(srcdir)/netwerk/dns/tests/unit/data/fake_public_suffix_list.dat backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/netwerk/dns/prepare_tlds.py main netwerk/dns/tests/unit/data/fake_remote_dafsa.bin netwerk/dns/tests/unit/data/$(MDDEPDIR)/fake_remote_dafsa.bin.pp netwerk/dns/tests/unit/data/$(MDDEPDIR)/fake_remote_dafsa.bin.stub $(srcdir)/netwerk/dns/tests/unit/data/fake_public_suffix_list.dat bin)
	@$(TOUCH) $@

netwerk/socket/neqo_glue/export: netwerk/socket/neqo_glue/$(MDDEPDIR)/neqo_glue_ffi_generated.h.stub
netwerk/socket/neqo_glue/neqo_glue_ffi_generated.h: netwerk/socket/neqo_glue/$(MDDEPDIR)/neqo_glue_ffi_generated.h.stub ;
EXTRA_MDDEPEND_FILES += netwerk/socket/neqo_glue/$(MDDEPDIR)/neqo_glue_ffi_generated.h.pp
netwerk/socket/neqo_glue/$(MDDEPDIR)/neqo_glue_ffi_generated.h.stub: /worker/build/build/RunCbindgen.py config/cbindgen-metadata.json $(srcdir)/netwerk/socket/neqo_glue
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate netwerk/socket/neqo_glue/neqo_glue_ffi_generated.h netwerk/socket/neqo_glue/$(MDDEPDIR)/neqo_glue_ffi_generated.h.pp netwerk/socket/neqo_glue/$(MDDEPDIR)/neqo_glue_ffi_generated.h.stub config/cbindgen-metadata.json $(srcdir)/netwerk/socket/neqo_glue)
	@$(TOUCH) $@

gfx/webrender_bindings/export: gfx/webrender_bindings/$(MDDEPDIR)/webrender_ffi_generated.h.stub
gfx/webrender_bindings/webrender_ffi_generated.h: gfx/webrender_bindings/$(MDDEPDIR)/webrender_ffi_generated.h.stub ;
EXTRA_MDDEPEND_FILES += gfx/webrender_bindings/$(MDDEPDIR)/webrender_ffi_generated.h.pp
gfx/webrender_bindings/$(MDDEPDIR)/webrender_ffi_generated.h.stub: /worker/build/build/RunCbindgen.py config/cbindgen-metadata.json $(srcdir)/gfx/webrender_bindings $(srcdir)/gfx/wr/webrender $(srcdir)/gfx/wr/webrender_api
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate gfx/webrender_bindings/webrender_ffi_generated.h gfx/webrender_bindings/$(MDDEPDIR)/webrender_ffi_generated.h.pp gfx/webrender_bindings/$(MDDEPDIR)/webrender_ffi_generated.h.stub config/cbindgen-metadata.json $(srcdir)/gfx/webrender_bindings $(srcdir)/gfx/wr/webrender $(srcdir)/gfx/wr/webrender_api)
	@$(TOUCH) $@

gfx/wgpu_bindings/export: gfx/wgpu_bindings/$(MDDEPDIR)/wgpu_ffi_generated.h.stub
gfx/wgpu_bindings/wgpu_ffi_generated.h: gfx/wgpu_bindings/$(MDDEPDIR)/wgpu_ffi_generated.h.stub ;
EXTRA_MDDEPEND_FILES += gfx/wgpu_bindings/$(MDDEPDIR)/wgpu_ffi_generated.h.pp
gfx/wgpu_bindings/$(MDDEPDIR)/wgpu_ffi_generated.h.stub: /worker/build/build/RunCbindgen.py config/cbindgen-metadata.json $(srcdir)/gfx/wgpu_bindings $(srcdir)/gfx/wgpu
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate gfx/wgpu_bindings/wgpu_ffi_generated.h gfx/wgpu_bindings/$(MDDEPDIR)/wgpu_ffi_generated.h.pp gfx/wgpu_bindings/$(MDDEPDIR)/wgpu_ffi_generated.h.stub config/cbindgen-metadata.json $(srcdir)/gfx/wgpu_bindings $(srcdir)/gfx/wgpu)
	@$(TOUCH) $@

dom/base/export: dom/base/$(MDDEPDIR)/UseCounterList.h.stub
dom/base/UseCounterList.h: dom/base/$(MDDEPDIR)/UseCounterList.h.stub ;
EXTRA_MDDEPEND_FILES += dom/base/$(MDDEPDIR)/UseCounterList.h.pp
dom/base/$(MDDEPDIR)/UseCounterList.h.stub: /worker/build/dom/base/gen-usecounters.py $(srcdir)/dom/base/UseCounters.conf
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/dom/base/gen-usecounters.py use_counter_list dom/base/UseCounterList.h dom/base/$(MDDEPDIR)/UseCounterList.h.pp dom/base/$(MDDEPDIR)/UseCounterList.h.stub $(srcdir)/dom/base/UseCounters.conf)
	@$(TOUCH) $@

dom/base/export: dom/base/$(MDDEPDIR)/UseCounterWorkerList.h.stub
dom/base/UseCounterWorkerList.h: dom/base/$(MDDEPDIR)/UseCounterWorkerList.h.stub ;
EXTRA_MDDEPEND_FILES += dom/base/$(MDDEPDIR)/UseCounterWorkerList.h.pp
dom/base/$(MDDEPDIR)/UseCounterWorkerList.h.stub: /worker/build/dom/base/gen-usecounters.py $(srcdir)/dom/base/UseCountersWorker.conf
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/dom/base/gen-usecounters.py use_counter_list dom/base/UseCounterWorkerList.h dom/base/$(MDDEPDIR)/UseCounterWorkerList.h.pp dom/base/$(MDDEPDIR)/UseCounterWorkerList.h.stub $(srcdir)/dom/base/UseCountersWorker.conf)
	@$(TOUCH) $@

dom/bindings/export: dom/bindings/$(MDDEPDIR)/CSS2Properties.webidl.stub
dom/bindings/CSS2Properties.webidl: dom/bindings/$(MDDEPDIR)/CSS2Properties.webidl.stub ;
EXTRA_MDDEPEND_FILES += dom/bindings/$(MDDEPDIR)/CSS2Properties.webidl.pp
dom/bindings/$(MDDEPDIR)/CSS2Properties.webidl.stub: /worker/build/dom/bindings/GenerateCSS2PropertiesWebIDL.py $(srcdir)/dom/webidl/CSS2Properties.webidl.in layout/style/ServoCSSPropList.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/dom/bindings/GenerateCSS2PropertiesWebIDL.py generate dom/bindings/CSS2Properties.webidl dom/bindings/$(MDDEPDIR)/CSS2Properties.webidl.pp dom/bindings/$(MDDEPDIR)/CSS2Properties.webidl.stub $(srcdir)/dom/webidl/CSS2Properties.webidl.in layout/style/ServoCSSPropList.py)
	@$(TOUCH) $@

dom/media/export: dom/media/$(MDDEPDIR)/audioipc_client_ffi_generated.h.stub
dom/media/audioipc_client_ffi_generated.h: dom/media/$(MDDEPDIR)/audioipc_client_ffi_generated.h.stub ;
EXTRA_MDDEPEND_FILES += dom/media/$(MDDEPDIR)/audioipc_client_ffi_generated.h.pp
dom/media/$(MDDEPDIR)/audioipc_client_ffi_generated.h.stub: /worker/build/build/RunCbindgen.py config/cbindgen-metadata.json $(srcdir)/third_party/rust/audioipc-client
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate dom/media/audioipc_client_ffi_generated.h dom/media/$(MDDEPDIR)/audioipc_client_ffi_generated.h.pp dom/media/$(MDDEPDIR)/audioipc_client_ffi_generated.h.stub config/cbindgen-metadata.json $(srcdir)/third_party/rust/audioipc-client)
	@$(TOUCH) $@

dom/media/export: dom/media/$(MDDEPDIR)/audioipc_server_ffi_generated.h.stub
dom/media/audioipc_server_ffi_generated.h: dom/media/$(MDDEPDIR)/audioipc_server_ffi_generated.h.stub ;
EXTRA_MDDEPEND_FILES += dom/media/$(MDDEPDIR)/audioipc_server_ffi_generated.h.pp
dom/media/$(MDDEPDIR)/audioipc_server_ffi_generated.h.stub: /worker/build/build/RunCbindgen.py config/cbindgen-metadata.json $(srcdir)/third_party/rust/audioipc-server
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate dom/media/audioipc_server_ffi_generated.h dom/media/$(MDDEPDIR)/audioipc_server_ffi_generated.h.pp dom/media/$(MDDEPDIR)/audioipc_server_ffi_generated.h.stub config/cbindgen-metadata.json $(srcdir)/third_party/rust/audioipc-server)
	@$(TOUCH) $@

layout/style/export: layout/style/$(MDDEPDIR)/nsCSSPropertyID.h.stub
layout/style/nsCSSPropertyID.h: layout/style/$(MDDEPDIR)/nsCSSPropertyID.h.stub ;
EXTRA_MDDEPEND_FILES += layout/style/$(MDDEPDIR)/nsCSSPropertyID.h.pp
layout/style/$(MDDEPDIR)/nsCSSPropertyID.h.stub: /worker/build/layout/style/GenerateCSSPropertyID.py $(srcdir)/layout/style/nsCSSPropertyID.h.in layout/style/ServoCSSPropList.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/layout/style/GenerateCSSPropertyID.py generate layout/style/nsCSSPropertyID.h layout/style/$(MDDEPDIR)/nsCSSPropertyID.h.pp layout/style/$(MDDEPDIR)/nsCSSPropertyID.h.stub $(srcdir)/layout/style/nsCSSPropertyID.h.in layout/style/ServoCSSPropList.py)
	@$(TOUCH) $@

layout/style/export: layout/style/$(MDDEPDIR)/ServoCSSPropList.h.stub
layout/style/ServoCSSPropList.h: layout/style/$(MDDEPDIR)/ServoCSSPropList.h.stub ;
EXTRA_MDDEPEND_FILES += layout/style/$(MDDEPDIR)/ServoCSSPropList.h.pp
layout/style/$(MDDEPDIR)/ServoCSSPropList.h.stub: /worker/build/layout/style/GenerateServoCSSPropList.py layout/style/ServoCSSPropList.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/layout/style/GenerateServoCSSPropList.py generate_header layout/style/ServoCSSPropList.h layout/style/$(MDDEPDIR)/ServoCSSPropList.h.pp layout/style/$(MDDEPDIR)/ServoCSSPropList.h.stub layout/style/ServoCSSPropList.py)
	@$(TOUCH) $@

layout/style/export: layout/style/$(MDDEPDIR)/ServoCSSPropList.py.stub
layout/style/ServoCSSPropList.py: layout/style/$(MDDEPDIR)/ServoCSSPropList.py.stub ;
EXTRA_MDDEPEND_FILES += layout/style/$(MDDEPDIR)/ServoCSSPropList.py.pp
layout/style/$(MDDEPDIR)/ServoCSSPropList.py.stub: /worker/build/layout/style/GenerateServoCSSPropList.py $(srcdir)/layout/style/ServoCSSPropList.mako.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/layout/style/GenerateServoCSSPropList.py generate_data layout/style/ServoCSSPropList.py layout/style/$(MDDEPDIR)/ServoCSSPropList.py.pp layout/style/$(MDDEPDIR)/ServoCSSPropList.py.stub $(srcdir)/layout/style/ServoCSSPropList.mako.py)
	@$(TOUCH) $@

layout/style/export: layout/style/$(MDDEPDIR)/CompositorAnimatableProperties.h.stub
layout/style/CompositorAnimatableProperties.h: layout/style/$(MDDEPDIR)/CompositorAnimatableProperties.h.stub ;
EXTRA_MDDEPEND_FILES += layout/style/$(MDDEPDIR)/CompositorAnimatableProperties.h.pp
layout/style/$(MDDEPDIR)/CompositorAnimatableProperties.h.stub: /worker/build/layout/style/GenerateCompositorAnimatableProperties.py layout/style/ServoCSSPropList.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/layout/style/GenerateCompositorAnimatableProperties.py generate layout/style/CompositorAnimatableProperties.h layout/style/$(MDDEPDIR)/CompositorAnimatableProperties.h.pp layout/style/$(MDDEPDIR)/CompositorAnimatableProperties.h.stub layout/style/ServoCSSPropList.py)
	@$(TOUCH) $@

layout/style/export: layout/style/$(MDDEPDIR)/CountedUnknownProperties.h.stub
layout/style/CountedUnknownProperties.h: layout/style/$(MDDEPDIR)/CountedUnknownProperties.h.stub ;
EXTRA_MDDEPEND_FILES += layout/style/$(MDDEPDIR)/CountedUnknownProperties.h.pp
layout/style/$(MDDEPDIR)/CountedUnknownProperties.h.stub: /worker/build/layout/style/GenerateCountedUnknownProperties.py $(srcdir)/servo/components/style/properties/counted_unknown_properties.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/layout/style/GenerateCountedUnknownProperties.py generate layout/style/CountedUnknownProperties.h layout/style/$(MDDEPDIR)/CountedUnknownProperties.h.pp layout/style/$(MDDEPDIR)/CountedUnknownProperties.h.stub $(srcdir)/servo/components/style/properties/counted_unknown_properties.py)
	@$(TOUCH) $@

layout/style/export: layout/style/$(MDDEPDIR)/ServoStyleConsts.h.stub
layout/style/ServoStyleConsts.h: layout/style/$(MDDEPDIR)/ServoStyleConsts.h.stub ;
EXTRA_MDDEPEND_FILES += layout/style/$(MDDEPDIR)/ServoStyleConsts.h.pp
layout/style/$(MDDEPDIR)/ServoStyleConsts.h.stub: /worker/build/build/RunCbindgen.py config/cbindgen-metadata.json $(srcdir)/servo/ports/geckolib $(srcdir)/servo/components/style
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate layout/style/ServoStyleConsts.h layout/style/$(MDDEPDIR)/ServoStyleConsts.h.pp layout/style/$(MDDEPDIR)/ServoStyleConsts.h.stub config/cbindgen-metadata.json $(srcdir)/servo/ports/geckolib $(srcdir)/servo/components/style)
	@$(TOUCH) $@

layout/style/test/misc: layout/style/test/$(MDDEPDIR)/css_properties.js.stub
layout/style/test/css_properties.js: layout/style/test/$(MDDEPDIR)/css_properties.js.stub ;
EXTRA_MDDEPEND_FILES += layout/style/test/$(MDDEPDIR)/css_properties.js.pp
layout/style/test/$(MDDEPDIR)/css_properties.js.stub: /worker/build/layout/style/test/gen-css-properties.py $(srcdir)/layout/style/test/css_properties_like_longhand.js layout/style/test/host_ListCSSProperties
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/layout/style/test/gen-css-properties.py main layout/style/test/css_properties.js layout/style/test/$(MDDEPDIR)/css_properties.js.pp layout/style/test/$(MDDEPDIR)/css_properties.js.stub $(srcdir)/layout/style/test/css_properties_like_longhand.js layout/style/test/host_ListCSSProperties)
	@$(TOUCH) $@

layout/style/test/gtest/export: layout/style/test/gtest/$(MDDEPDIR)/ExampleStylesheet.h.stub
layout/style/test/gtest/ExampleStylesheet.h: layout/style/test/gtest/$(MDDEPDIR)/ExampleStylesheet.h.stub ;
EXTRA_MDDEPEND_FILES += layout/style/test/gtest/$(MDDEPDIR)/ExampleStylesheet.h.pp
layout/style/test/gtest/$(MDDEPDIR)/ExampleStylesheet.h.stub: /worker/build/layout/style/test/gtest/generate_example_stylesheet.py $(srcdir)/layout/style/test/gtest/example.css
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/layout/style/test/gtest/generate_example_stylesheet.py main layout/style/test/gtest/ExampleStylesheet.h layout/style/test/gtest/$(MDDEPDIR)/ExampleStylesheet.h.pp layout/style/test/gtest/$(MDDEPDIR)/ExampleStylesheet.h.stub $(srcdir)/layout/style/test/gtest/example.css)
	@$(TOUCH) $@

layout/generic/export: layout/generic/$(MDDEPDIR)/FrameIdList.h.stub
layout/generic/FrameIdList.h: layout/generic/$(MDDEPDIR)/FrameIdList.h.stub ;
EXTRA_MDDEPEND_FILES += layout/generic/$(MDDEPDIR)/FrameIdList.h.pp
layout/generic/$(MDDEPDIR)/FrameIdList.h.stub: /worker/build/layout/generic/GenerateFrameLists.py $(srcdir)/layout/generic/FrameClasses.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/layout/generic/GenerateFrameLists.py generate_frame_id_list_h layout/generic/FrameIdList.h layout/generic/$(MDDEPDIR)/FrameIdList.h.pp layout/generic/$(MDDEPDIR)/FrameIdList.h.stub $(srcdir)/layout/generic/FrameClasses.py)
	@$(TOUCH) $@

layout/generic/export: layout/generic/$(MDDEPDIR)/FrameTypeList.h.stub
layout/generic/FrameTypeList.h: layout/generic/$(MDDEPDIR)/FrameTypeList.h.stub ;
EXTRA_MDDEPEND_FILES += layout/generic/$(MDDEPDIR)/FrameTypeList.h.pp
layout/generic/$(MDDEPDIR)/FrameTypeList.h.stub: /worker/build/layout/generic/GenerateFrameLists.py $(srcdir)/layout/generic/FrameClasses.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/layout/generic/GenerateFrameLists.py generate_frame_type_list_h layout/generic/FrameTypeList.h layout/generic/$(MDDEPDIR)/FrameTypeList.h.pp layout/generic/$(MDDEPDIR)/FrameTypeList.h.stub $(srcdir)/layout/generic/FrameClasses.py)
	@$(TOUCH) $@

toolkit/components/featuregates/misc: toolkit/components/featuregates/$(MDDEPDIR)/feature_definitions.json.stub
toolkit/components/featuregates/feature_definitions.json: toolkit/components/featuregates/$(MDDEPDIR)/feature_definitions.json.stub ;
EXTRA_MDDEPEND_FILES += toolkit/components/featuregates/$(MDDEPDIR)/feature_definitions.json.pp
toolkit/components/featuregates/$(MDDEPDIR)/feature_definitions.json.stub: /worker/build/toolkit/components/featuregates/gen_feature_definitions.py $(srcdir)/toolkit/components/featuregates/Features.toml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/featuregates/gen_feature_definitions.py main toolkit/components/featuregates/feature_definitions.json toolkit/components/featuregates/$(MDDEPDIR)/feature_definitions.json.pp toolkit/components/featuregates/$(MDDEPDIR)/feature_definitions.json.stub $(srcdir)/toolkit/components/featuregates/Features.toml)
	@$(TOUCH) $@

toolkit/components/glean/export: toolkit/components/glean/$(MDDEPDIR)/fog_ffi_generated.h.stub
toolkit/components/glean/fog_ffi_generated.h: toolkit/components/glean/$(MDDEPDIR)/fog_ffi_generated.h.stub ;
EXTRA_MDDEPEND_FILES += toolkit/components/glean/$(MDDEPDIR)/fog_ffi_generated.h.pp
toolkit/components/glean/$(MDDEPDIR)/fog_ffi_generated.h.stub: /worker/build/build/RunCbindgen.py config/cbindgen-metadata.json $(srcdir)/toolkit/components/glean
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate toolkit/components/glean/fog_ffi_generated.h toolkit/components/glean/$(MDDEPDIR)/fog_ffi_generated.h.pp toolkit/components/glean/$(MDDEPDIR)/fog_ffi_generated.h.stub config/cbindgen-metadata.json $(srcdir)/toolkit/components/glean)
	@$(TOUCH) $@

toolkit/components/glean/export: toolkit/components/glean/api/src/$(MDDEPDIR)/metrics.rs.stub
toolkit/components/glean/api/src/metrics.rs: toolkit/components/glean/api/src/$(MDDEPDIR)/metrics.rs.stub ;
EXTRA_MDDEPEND_FILES += toolkit/components/glean/api/src/$(MDDEPDIR)/metrics.rs.pp
toolkit/components/glean/api/src/$(MDDEPDIR)/metrics.rs.stub: /worker/build/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(srcdir)/toolkit/components/glean/metrics_index.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/cpp.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/js.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/rust.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/string_table.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/util.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp_pings.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js_pings.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust_pings.jinja2 $(srcdir)/toolkit/components/glean/metrics.yaml $(srcdir)/toolkit/components/glean/test_metrics.yaml backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py main toolkit/components/glean/api/src/metrics.rs toolkit/components/glean/api/src/$(MDDEPDIR)/metrics.rs.pp toolkit/components/glean/api/src/$(MDDEPDIR)/metrics.rs.stub $(srcdir)/toolkit/components/glean/metrics_index.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/cpp.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/js.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/rust.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/string_table.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/util.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp_pings.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js_pings.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust_pings.jinja2 $(srcdir)/toolkit/components/glean/metrics.yaml $(srcdir)/toolkit/components/glean/test_metrics.yaml 87.0)
	@$(TOUCH) $@

toolkit/components/glean/export: toolkit/components/glean/$(MDDEPDIR)/GleanMetrics.h.stub
toolkit/components/glean/GleanMetrics.h: toolkit/components/glean/$(MDDEPDIR)/GleanMetrics.h.stub ;
EXTRA_MDDEPEND_FILES += toolkit/components/glean/$(MDDEPDIR)/GleanMetrics.h.pp
toolkit/components/glean/$(MDDEPDIR)/GleanMetrics.h.stub: /worker/build/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(srcdir)/toolkit/components/glean/metrics_index.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/cpp.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/js.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/rust.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/string_table.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/util.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp_pings.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js_pings.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust_pings.jinja2 $(srcdir)/toolkit/components/glean/metrics.yaml $(srcdir)/toolkit/components/glean/test_metrics.yaml backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py cpp_metrics toolkit/components/glean/GleanMetrics.h toolkit/components/glean/$(MDDEPDIR)/GleanMetrics.h.pp toolkit/components/glean/$(MDDEPDIR)/GleanMetrics.h.stub $(srcdir)/toolkit/components/glean/metrics_index.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/cpp.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/js.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/rust.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/string_table.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/util.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp_pings.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js_pings.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust_pings.jinja2 $(srcdir)/toolkit/components/glean/metrics.yaml $(srcdir)/toolkit/components/glean/test_metrics.yaml 87.0)
	@$(TOUCH) $@

toolkit/components/glean/export: toolkit/components/glean/$(MDDEPDIR)/GleanJSMetricsLookup.h.stub
toolkit/components/glean/GleanJSMetricsLookup.h: toolkit/components/glean/$(MDDEPDIR)/GleanJSMetricsLookup.h.stub ;
EXTRA_MDDEPEND_FILES += toolkit/components/glean/$(MDDEPDIR)/GleanJSMetricsLookup.h.pp
toolkit/components/glean/$(MDDEPDIR)/GleanJSMetricsLookup.h.stub: /worker/build/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(srcdir)/toolkit/components/glean/metrics_index.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/cpp.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/js.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/rust.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/string_table.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/util.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp_pings.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js_pings.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust_pings.jinja2 $(srcdir)/toolkit/components/glean/metrics.yaml $(srcdir)/toolkit/components/glean/test_metrics.yaml backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py js_metrics toolkit/components/glean/GleanJSMetricsLookup.h toolkit/components/glean/$(MDDEPDIR)/GleanJSMetricsLookup.h.pp toolkit/components/glean/$(MDDEPDIR)/GleanJSMetricsLookup.h.stub $(srcdir)/toolkit/components/glean/metrics_index.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/cpp.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/js.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/rust.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/string_table.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/util.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp_pings.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js_pings.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust_pings.jinja2 $(srcdir)/toolkit/components/glean/metrics.yaml $(srcdir)/toolkit/components/glean/test_metrics.yaml 87.0)
	@$(TOUCH) $@

toolkit/components/glean/export: toolkit/components/glean/api/src/$(MDDEPDIR)/pings.rs.stub
toolkit/components/glean/api/src/pings.rs: toolkit/components/glean/api/src/$(MDDEPDIR)/pings.rs.stub ;
EXTRA_MDDEPEND_FILES += toolkit/components/glean/api/src/$(MDDEPDIR)/pings.rs.pp
toolkit/components/glean/api/src/$(MDDEPDIR)/pings.rs.stub: /worker/build/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(srcdir)/toolkit/components/glean/metrics_index.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/cpp.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/js.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/rust.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/string_table.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/util.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp_pings.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js_pings.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust_pings.jinja2 $(srcdir)/toolkit/components/glean/pings.yaml $(srcdir)/toolkit/components/glean/test_pings.yaml backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py main toolkit/components/glean/api/src/pings.rs toolkit/components/glean/api/src/$(MDDEPDIR)/pings.rs.pp toolkit/components/glean/api/src/$(MDDEPDIR)/pings.rs.stub $(srcdir)/toolkit/components/glean/metrics_index.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/cpp.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/js.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/rust.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/string_table.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/util.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp_pings.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js_pings.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust_pings.jinja2 $(srcdir)/toolkit/components/glean/pings.yaml $(srcdir)/toolkit/components/glean/test_pings.yaml 87.0)
	@$(TOUCH) $@

toolkit/components/glean/export: toolkit/components/glean/$(MDDEPDIR)/GleanPings.h.stub
toolkit/components/glean/GleanPings.h: toolkit/components/glean/$(MDDEPDIR)/GleanPings.h.stub ;
EXTRA_MDDEPEND_FILES += toolkit/components/glean/$(MDDEPDIR)/GleanPings.h.pp
toolkit/components/glean/$(MDDEPDIR)/GleanPings.h.stub: /worker/build/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(srcdir)/toolkit/components/glean/metrics_index.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/cpp.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/js.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/rust.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/string_table.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/util.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp_pings.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js_pings.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust_pings.jinja2 $(srcdir)/toolkit/components/glean/pings.yaml $(srcdir)/toolkit/components/glean/test_pings.yaml backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py cpp_metrics toolkit/components/glean/GleanPings.h toolkit/components/glean/$(MDDEPDIR)/GleanPings.h.pp toolkit/components/glean/$(MDDEPDIR)/GleanPings.h.stub $(srcdir)/toolkit/components/glean/metrics_index.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/cpp.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/js.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/rust.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/string_table.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/util.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp_pings.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js_pings.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust_pings.jinja2 $(srcdir)/toolkit/components/glean/pings.yaml $(srcdir)/toolkit/components/glean/test_pings.yaml 87.0)
	@$(TOUCH) $@

toolkit/components/glean/export: toolkit/components/glean/$(MDDEPDIR)/GleanJSPingsLookup.h.stub
toolkit/components/glean/GleanJSPingsLookup.h: toolkit/components/glean/$(MDDEPDIR)/GleanJSPingsLookup.h.stub ;
EXTRA_MDDEPEND_FILES += toolkit/components/glean/$(MDDEPDIR)/GleanJSPingsLookup.h.pp
toolkit/components/glean/$(MDDEPDIR)/GleanJSPingsLookup.h.stub: /worker/build/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(srcdir)/toolkit/components/glean/metrics_index.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/cpp.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/js.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/rust.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/string_table.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/util.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp_pings.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js_pings.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust_pings.jinja2 $(srcdir)/toolkit/components/glean/pings.yaml $(srcdir)/toolkit/components/glean/test_pings.yaml backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py js_metrics toolkit/components/glean/GleanJSPingsLookup.h toolkit/components/glean/$(MDDEPDIR)/GleanJSPingsLookup.h.pp toolkit/components/glean/$(MDDEPDIR)/GleanJSPingsLookup.h.stub $(srcdir)/toolkit/components/glean/metrics_index.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/cpp.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/js.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/run_glean_parser.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/rust.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/string_table.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/util.py $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/cpp_pings.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/js_pings.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust.jinja2 $(srcdir)/toolkit/components/glean/build_scripts/glean_parser_ext/templates/rust_pings.jinja2 $(srcdir)/toolkit/components/glean/pings.yaml $(srcdir)/toolkit/components/glean/test_pings.yaml 87.0)
	@$(TOUCH) $@

toolkit/components/telemetry/export: toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramEnums.h.stub
toolkit/components/telemetry/TelemetryHistogramEnums.h: toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramEnums.h.stub ;
EXTRA_MDDEPEND_FILES += toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramEnums.h.pp
toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramEnums.h.stub: /worker/build/toolkit/components/telemetry/build_scripts/gen_histogram_enum.py $(srcdir)/toolkit/components/telemetry/Histograms.json $(srcdir)/dom/base/UseCounters.conf $(srcdir)/dom/base/nsDeprecatedOperationList.h layout/style/ServoCSSPropList.py $(srcdir)/servo/components/style/properties/counted_unknown_properties.py $(srcdir)/dom/base/UseCountersWorker.conf
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/telemetry/build_scripts/gen_histogram_enum.py main toolkit/components/telemetry/TelemetryHistogramEnums.h toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramEnums.h.pp toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramEnums.h.stub $(srcdir)/toolkit/components/telemetry/Histograms.json $(srcdir)/dom/base/UseCounters.conf $(srcdir)/dom/base/nsDeprecatedOperationList.h layout/style/ServoCSSPropList.py $(srcdir)/servo/components/style/properties/counted_unknown_properties.py $(srcdir)/dom/base/UseCountersWorker.conf)
	@$(TOUCH) $@

toolkit/components/telemetry/export: toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramNameMap.h.stub
toolkit/components/telemetry/TelemetryHistogramNameMap.h: toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramNameMap.h.stub ;
EXTRA_MDDEPEND_FILES += toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramNameMap.h.pp
toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramNameMap.h.stub: /worker/build/toolkit/components/telemetry/build_scripts/gen_histogram_phf.py $(srcdir)/toolkit/components/telemetry/Histograms.json $(srcdir)/dom/base/UseCounters.conf $(srcdir)/dom/base/nsDeprecatedOperationList.h layout/style/ServoCSSPropList.py $(srcdir)/servo/components/style/properties/counted_unknown_properties.py $(srcdir)/dom/base/UseCountersWorker.conf
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/telemetry/build_scripts/gen_histogram_phf.py main toolkit/components/telemetry/TelemetryHistogramNameMap.h toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramNameMap.h.pp toolkit/components/telemetry/$(MDDEPDIR)/TelemetryHistogramNameMap.h.stub $(srcdir)/toolkit/components/telemetry/Histograms.json $(srcdir)/dom/base/UseCounters.conf $(srcdir)/dom/base/nsDeprecatedOperationList.h layout/style/ServoCSSPropList.py $(srcdir)/servo/components/style/properties/counted_unknown_properties.py $(srcdir)/dom/base/UseCountersWorker.conf)
	@$(TOUCH) $@

toolkit/components/telemetry/export: toolkit/components/telemetry/$(MDDEPDIR)/TelemetryScalarData.h.stub
toolkit/components/telemetry/TelemetryScalarData.h: toolkit/components/telemetry/$(MDDEPDIR)/TelemetryScalarData.h.stub ;
EXTRA_MDDEPEND_FILES += toolkit/components/telemetry/$(MDDEPDIR)/TelemetryScalarData.h.pp
toolkit/components/telemetry/$(MDDEPDIR)/TelemetryScalarData.h.stub: /worker/build/toolkit/components/telemetry/build_scripts/gen_scalar_data.py $(srcdir)/toolkit/components/telemetry/Scalars.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/telemetry/build_scripts/gen_scalar_data.py main toolkit/components/telemetry/TelemetryScalarData.h toolkit/components/telemetry/$(MDDEPDIR)/TelemetryScalarData.h.pp toolkit/components/telemetry/$(MDDEPDIR)/TelemetryScalarData.h.stub $(srcdir)/toolkit/components/telemetry/Scalars.yaml)
	@$(TOUCH) $@

toolkit/components/telemetry/export: toolkit/components/telemetry/$(MDDEPDIR)/TelemetryScalarEnums.h.stub
toolkit/components/telemetry/TelemetryScalarEnums.h: toolkit/components/telemetry/$(MDDEPDIR)/TelemetryScalarEnums.h.stub ;
EXTRA_MDDEPEND_FILES += toolkit/components/telemetry/$(MDDEPDIR)/TelemetryScalarEnums.h.pp
toolkit/components/telemetry/$(MDDEPDIR)/TelemetryScalarEnums.h.stub: /worker/build/toolkit/components/telemetry/build_scripts/gen_scalar_enum.py $(srcdir)/toolkit/components/telemetry/Scalars.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/telemetry/build_scripts/gen_scalar_enum.py main toolkit/components/telemetry/TelemetryScalarEnums.h toolkit/components/telemetry/$(MDDEPDIR)/TelemetryScalarEnums.h.pp toolkit/components/telemetry/$(MDDEPDIR)/TelemetryScalarEnums.h.stub $(srcdir)/toolkit/components/telemetry/Scalars.yaml)
	@$(TOUCH) $@

toolkit/components/telemetry/misc: toolkit/components/telemetry/$(MDDEPDIR)/ScalarArtifactDefinitions.json.stub
toolkit/components/telemetry/ScalarArtifactDefinitions.json: toolkit/components/telemetry/$(MDDEPDIR)/ScalarArtifactDefinitions.json.stub ;
EXTRA_MDDEPEND_FILES += toolkit/components/telemetry/$(MDDEPDIR)/ScalarArtifactDefinitions.json.pp
toolkit/components/telemetry/$(MDDEPDIR)/ScalarArtifactDefinitions.json.stub: /worker/build/toolkit/components/telemetry/build_scripts/gen_scalar_data.py $(srcdir)/toolkit/components/telemetry/Scalars.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/telemetry/build_scripts/gen_scalar_data.py generate_JSON_definitions toolkit/components/telemetry/ScalarArtifactDefinitions.json toolkit/components/telemetry/$(MDDEPDIR)/ScalarArtifactDefinitions.json.pp toolkit/components/telemetry/$(MDDEPDIR)/ScalarArtifactDefinitions.json.stub $(srcdir)/toolkit/components/telemetry/Scalars.yaml)
	@$(TOUCH) $@

toolkit/components/telemetry/export: toolkit/components/telemetry/$(MDDEPDIR)/TelemetryEventData.h.stub
toolkit/components/telemetry/TelemetryEventData.h: toolkit/components/telemetry/$(MDDEPDIR)/TelemetryEventData.h.stub ;
EXTRA_MDDEPEND_FILES += toolkit/components/telemetry/$(MDDEPDIR)/TelemetryEventData.h.pp
toolkit/components/telemetry/$(MDDEPDIR)/TelemetryEventData.h.stub: /worker/build/toolkit/components/telemetry/build_scripts/gen_event_data.py $(srcdir)/toolkit/components/telemetry/Events.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/telemetry/build_scripts/gen_event_data.py main toolkit/components/telemetry/TelemetryEventData.h toolkit/components/telemetry/$(MDDEPDIR)/TelemetryEventData.h.pp toolkit/components/telemetry/$(MDDEPDIR)/TelemetryEventData.h.stub $(srcdir)/toolkit/components/telemetry/Events.yaml)
	@$(TOUCH) $@

toolkit/components/telemetry/export: toolkit/components/telemetry/$(MDDEPDIR)/TelemetryEventEnums.h.stub
toolkit/components/telemetry/TelemetryEventEnums.h: toolkit/components/telemetry/$(MDDEPDIR)/TelemetryEventEnums.h.stub ;
EXTRA_MDDEPEND_FILES += toolkit/components/telemetry/$(MDDEPDIR)/TelemetryEventEnums.h.pp
toolkit/components/telemetry/$(MDDEPDIR)/TelemetryEventEnums.h.stub: /worker/build/toolkit/components/telemetry/build_scripts/gen_event_enum.py $(srcdir)/toolkit/components/telemetry/Events.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/telemetry/build_scripts/gen_event_enum.py main toolkit/components/telemetry/TelemetryEventEnums.h toolkit/components/telemetry/$(MDDEPDIR)/TelemetryEventEnums.h.pp toolkit/components/telemetry/$(MDDEPDIR)/TelemetryEventEnums.h.stub $(srcdir)/toolkit/components/telemetry/Events.yaml)
	@$(TOUCH) $@

toolkit/components/telemetry/misc: toolkit/components/telemetry/$(MDDEPDIR)/EventArtifactDefinitions.json.stub
toolkit/components/telemetry/EventArtifactDefinitions.json: toolkit/components/telemetry/$(MDDEPDIR)/EventArtifactDefinitions.json.stub ;
EXTRA_MDDEPEND_FILES += toolkit/components/telemetry/$(MDDEPDIR)/EventArtifactDefinitions.json.pp
toolkit/components/telemetry/$(MDDEPDIR)/EventArtifactDefinitions.json.stub: /worker/build/toolkit/components/telemetry/build_scripts/gen_event_data.py $(srcdir)/toolkit/components/telemetry/Events.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/telemetry/build_scripts/gen_event_data.py generate_JSON_definitions toolkit/components/telemetry/EventArtifactDefinitions.json toolkit/components/telemetry/$(MDDEPDIR)/EventArtifactDefinitions.json.pp toolkit/components/telemetry/$(MDDEPDIR)/EventArtifactDefinitions.json.stub $(srcdir)/toolkit/components/telemetry/Events.yaml)
	@$(TOUCH) $@

toolkit/components/telemetry/export: toolkit/components/telemetry/$(MDDEPDIR)/TelemetryProcessEnums.h.stub
toolkit/components/telemetry/TelemetryProcessEnums.h: toolkit/components/telemetry/$(MDDEPDIR)/TelemetryProcessEnums.h.stub ;
EXTRA_MDDEPEND_FILES += toolkit/components/telemetry/$(MDDEPDIR)/TelemetryProcessEnums.h.pp
toolkit/components/telemetry/$(MDDEPDIR)/TelemetryProcessEnums.h.stub: /worker/build/toolkit/components/telemetry/build_scripts/gen_process_enum.py $(srcdir)/toolkit/components/telemetry/Processes.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/telemetry/build_scripts/gen_process_enum.py main toolkit/components/telemetry/TelemetryProcessEnums.h toolkit/components/telemetry/$(MDDEPDIR)/TelemetryProcessEnums.h.pp toolkit/components/telemetry/$(MDDEPDIR)/TelemetryProcessEnums.h.stub $(srcdir)/toolkit/components/telemetry/Processes.yaml)
	@$(TOUCH) $@

toolkit/components/telemetry/export: toolkit/components/telemetry/$(MDDEPDIR)/TelemetryProcessData.h.stub
toolkit/components/telemetry/TelemetryProcessData.h: toolkit/components/telemetry/$(MDDEPDIR)/TelemetryProcessData.h.stub ;
EXTRA_MDDEPEND_FILES += toolkit/components/telemetry/$(MDDEPDIR)/TelemetryProcessData.h.pp
toolkit/components/telemetry/$(MDDEPDIR)/TelemetryProcessData.h.stub: /worker/build/toolkit/components/telemetry/build_scripts/gen_process_data.py $(srcdir)/toolkit/components/telemetry/Processes.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/telemetry/build_scripts/gen_process_data.py main toolkit/components/telemetry/TelemetryProcessData.h toolkit/components/telemetry/$(MDDEPDIR)/TelemetryProcessData.h.pp toolkit/components/telemetry/$(MDDEPDIR)/TelemetryProcessData.h.stub $(srcdir)/toolkit/components/telemetry/Processes.yaml)
	@$(TOUCH) $@

toolkit/components/telemetry/misc: toolkit/components/telemetry/$(MDDEPDIR)/glean_checks.stub
toolkit/components/telemetry/glean_checks: toolkit/components/telemetry/$(MDDEPDIR)/glean_checks.stub ;
EXTRA_MDDEPEND_FILES += toolkit/components/telemetry/$(MDDEPDIR)/glean_checks.pp
toolkit/components/telemetry/$(MDDEPDIR)/glean_checks.stub: /worker/build/toolkit/components/telemetry/build_scripts/run_glean_parser.py $(srcdir)/toolkit/components/telemetry/geckoview/streaming/metrics.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/telemetry/build_scripts/run_glean_parser.py main toolkit/components/telemetry/glean_checks toolkit/components/telemetry/$(MDDEPDIR)/glean_checks.pp toolkit/components/telemetry/$(MDDEPDIR)/glean_checks.stub $(srcdir)/toolkit/components/telemetry/geckoview/streaming/metrics.yaml)
	@$(TOUCH) $@

toolkit/components/telemetry/export: toolkit/components/telemetry/$(MDDEPDIR)/TelemetryUserInteractionData.h.stub
toolkit/components/telemetry/TelemetryUserInteractionData.h: toolkit/components/telemetry/$(MDDEPDIR)/TelemetryUserInteractionData.h.stub ;
EXTRA_MDDEPEND_FILES += toolkit/components/telemetry/$(MDDEPDIR)/TelemetryUserInteractionData.h.pp
toolkit/components/telemetry/$(MDDEPDIR)/TelemetryUserInteractionData.h.stub: /worker/build/toolkit/components/telemetry/build_scripts/gen_userinteraction_data.py $(srcdir)/toolkit/components/telemetry/UserInteractions.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/telemetry/build_scripts/gen_userinteraction_data.py main toolkit/components/telemetry/TelemetryUserInteractionData.h toolkit/components/telemetry/$(MDDEPDIR)/TelemetryUserInteractionData.h.pp toolkit/components/telemetry/$(MDDEPDIR)/TelemetryUserInteractionData.h.stub $(srcdir)/toolkit/components/telemetry/UserInteractions.yaml)
	@$(TOUCH) $@

toolkit/components/telemetry/export: toolkit/components/telemetry/$(MDDEPDIR)/TelemetryUserInteractionNameMap.h.stub
toolkit/components/telemetry/TelemetryUserInteractionNameMap.h: toolkit/components/telemetry/$(MDDEPDIR)/TelemetryUserInteractionNameMap.h.stub ;
EXTRA_MDDEPEND_FILES += toolkit/components/telemetry/$(MDDEPDIR)/TelemetryUserInteractionNameMap.h.pp
toolkit/components/telemetry/$(MDDEPDIR)/TelemetryUserInteractionNameMap.h.stub: /worker/build/toolkit/components/telemetry/build_scripts/gen_userinteraction_phf.py $(srcdir)/toolkit/components/telemetry/UserInteractions.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/telemetry/build_scripts/gen_userinteraction_phf.py main toolkit/components/telemetry/TelemetryUserInteractionNameMap.h toolkit/components/telemetry/$(MDDEPDIR)/TelemetryUserInteractionNameMap.h.pp toolkit/components/telemetry/$(MDDEPDIR)/TelemetryUserInteractionNameMap.h.stub $(srcdir)/toolkit/components/telemetry/UserInteractions.yaml)
	@$(TOUCH) $@

toolkit/components/normandy/test/browser/misc: toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-a-1.0.xpi.stub
toolkit/components/normandy/test/browser/addons/normandydriver-a-1.0.xpi: toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-a-1.0.xpi.stub ;
EXTRA_MDDEPEND_FILES += toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-a-1.0.xpi.pp
toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-a-1.0.xpi.stub: /worker/build/toolkit/components/normandy/test/create_xpi.py $(srcdir)/toolkit/components/normandy/test/browser/addons/normandydriver-a-1.0
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/normandy/test/create_xpi.py main toolkit/components/normandy/test/browser/addons/normandydriver-a-1.0.xpi toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-a-1.0.xpi.pp toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-a-1.0.xpi.stub $(srcdir)/toolkit/components/normandy/test/browser/addons/normandydriver-a-1.0)
	@$(TOUCH) $@

toolkit/components/normandy/test/browser/misc: toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-b-1.0.xpi.stub
toolkit/components/normandy/test/browser/addons/normandydriver-b-1.0.xpi: toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-b-1.0.xpi.stub ;
EXTRA_MDDEPEND_FILES += toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-b-1.0.xpi.pp
toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-b-1.0.xpi.stub: /worker/build/toolkit/components/normandy/test/create_xpi.py $(srcdir)/toolkit/components/normandy/test/browser/addons/normandydriver-b-1.0
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/normandy/test/create_xpi.py main toolkit/components/normandy/test/browser/addons/normandydriver-b-1.0.xpi toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-b-1.0.xpi.pp toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-b-1.0.xpi.stub $(srcdir)/toolkit/components/normandy/test/browser/addons/normandydriver-b-1.0)
	@$(TOUCH) $@

toolkit/components/normandy/test/browser/misc: toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-a-2.0.xpi.stub
toolkit/components/normandy/test/browser/addons/normandydriver-a-2.0.xpi: toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-a-2.0.xpi.stub ;
EXTRA_MDDEPEND_FILES += toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-a-2.0.xpi.pp
toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-a-2.0.xpi.stub: /worker/build/toolkit/components/normandy/test/create_xpi.py $(srcdir)/toolkit/components/normandy/test/browser/addons/normandydriver-a-2.0
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/components/normandy/test/create_xpi.py main toolkit/components/normandy/test/browser/addons/normandydriver-a-2.0.xpi toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-a-2.0.xpi.pp toolkit/components/normandy/test/browser/addons/$(MDDEPDIR)/normandydriver-a-2.0.xpi.stub $(srcdir)/toolkit/components/normandy/test/browser/addons/normandydriver-a-2.0)
	@$(TOUCH) $@

toolkit/crashreporter/export: toolkit/crashreporter/$(MDDEPDIR)/CrashAnnotations.h.stub
toolkit/crashreporter/CrashAnnotations.h: toolkit/crashreporter/$(MDDEPDIR)/CrashAnnotations.h.stub ;
EXTRA_MDDEPEND_FILES += toolkit/crashreporter/$(MDDEPDIR)/CrashAnnotations.h.pp
toolkit/crashreporter/$(MDDEPDIR)/CrashAnnotations.h.stub: /worker/build/toolkit/crashreporter/generate_crash_reporter_sources.py $(srcdir)/toolkit/crashreporter/CrashAnnotations.h.in $(srcdir)/toolkit/crashreporter/CrashAnnotations.yaml
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/crashreporter/generate_crash_reporter_sources.py emit_header toolkit/crashreporter/CrashAnnotations.h toolkit/crashreporter/$(MDDEPDIR)/CrashAnnotations.h.pp toolkit/crashreporter/$(MDDEPDIR)/CrashAnnotations.h.stub $(srcdir)/toolkit/crashreporter/CrashAnnotations.h.in $(srcdir)/toolkit/crashreporter/CrashAnnotations.yaml)
	@$(TOUCH) $@

toolkit/crashreporter/rust_minidump_writer_linux/export: toolkit/crashreporter/rust_minidump_writer_linux/$(MDDEPDIR)/rust_minidump_writer_linux_ffi_generated.h.stub
toolkit/crashreporter/rust_minidump_writer_linux/rust_minidump_writer_linux_ffi_generated.h: toolkit/crashreporter/rust_minidump_writer_linux/$(MDDEPDIR)/rust_minidump_writer_linux_ffi_generated.h.stub ;
EXTRA_MDDEPEND_FILES += toolkit/crashreporter/rust_minidump_writer_linux/$(MDDEPDIR)/rust_minidump_writer_linux_ffi_generated.h.pp
toolkit/crashreporter/rust_minidump_writer_linux/$(MDDEPDIR)/rust_minidump_writer_linux_ffi_generated.h.stub: /worker/build/build/RunCbindgen.py config/cbindgen-metadata.json $(srcdir)/toolkit/crashreporter/rust_minidump_writer_linux
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/build/RunCbindgen.py generate toolkit/crashreporter/rust_minidump_writer_linux/rust_minidump_writer_linux_ffi_generated.h toolkit/crashreporter/rust_minidump_writer_linux/$(MDDEPDIR)/rust_minidump_writer_linux_ffi_generated.h.pp toolkit/crashreporter/rust_minidump_writer_linux/$(MDDEPDIR)/rust_minidump_writer_linux_ffi_generated.h.stub config/cbindgen-metadata.json $(srcdir)/toolkit/crashreporter/rust_minidump_writer_linux)
	@$(TOUCH) $@

toolkit/locales/misc: toolkit/locales/$(MDDEPDIR)/multilocale.txt.stub
toolkit/locales/multilocale.txt: toolkit/locales/$(MDDEPDIR)/multilocale.txt.stub ;
EXTRA_MDDEPEND_FILES += toolkit/locales/$(MDDEPDIR)/multilocale.txt.pp
toolkit/locales/$(MDDEPDIR)/multilocale.txt.stub: /worker/build/toolkit/locales/gen_multilocale.py
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/locales/gen_multilocale.py main toolkit/locales/multilocale.txt toolkit/locales/$(MDDEPDIR)/multilocale.txt.pp toolkit/locales/$(MDDEPDIR)/multilocale.txt.stub)
	@$(TOUCH) $@

toolkit/mozapps/extensions/misc: toolkit/mozapps/extensions/$(MDDEPDIR)/built_in_addons.json.stub
toolkit/mozapps/extensions/built_in_addons.json: toolkit/mozapps/extensions/$(MDDEPDIR)/built_in_addons.json.stub ;
EXTRA_MDDEPEND_FILES += toolkit/mozapps/extensions/$(MDDEPDIR)/built_in_addons.json.pp
toolkit/mozapps/extensions/$(MDDEPDIR)/built_in_addons.json.stub: /worker/build/toolkit/mozapps/extensions/gen_built_in_addons.py backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/extensions/gen_built_in_addons.py main toolkit/mozapps/extensions/built_in_addons.json toolkit/mozapps/extensions/$(MDDEPDIR)/built_in_addons.json.pp toolkit/mozapps/extensions/$(MDDEPDIR)/built_in_addons.json.stub --features=browser/features)
	@$(TOUCH) $@

toolkit/mozapps/extensions/test/browser/misc: toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop1.xpi.stub
toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop1.xpi: toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop1.xpi.stub ;
EXTRA_MDDEPEND_FILES += toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop1.xpi.pp
toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop1.xpi.stub: /worker/build/toolkit/mozapps/extensions/test/create_xpi.py $(srcdir)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop1
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/extensions/test/create_xpi.py main toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop1.xpi toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop1.xpi.pp toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop1.xpi.stub $(srcdir)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop1)
	@$(TOUCH) $@

toolkit/mozapps/extensions/test/browser/misc: toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop1.zip.stub
toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop1.zip: toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop1.zip.stub ;
EXTRA_MDDEPEND_FILES += toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop1.zip.pp
toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop1.zip.stub: /worker/build/toolkit/mozapps/extensions/test/create_xpi.py $(srcdir)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop1
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/extensions/test/create_xpi.py main toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop1.zip toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop1.zip.pp toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop1.zip.stub $(srcdir)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop1)
	@$(TOUCH) $@

toolkit/mozapps/extensions/test/browser/misc: toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop2.xpi.stub
toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop2.xpi: toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop2.xpi.stub ;
EXTRA_MDDEPEND_FILES += toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop2.xpi.pp
toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop2.xpi.stub: /worker/build/toolkit/mozapps/extensions/test/create_xpi.py $(srcdir)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop2
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/extensions/test/create_xpi.py main toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop2.xpi toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop2.xpi.pp toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop2.xpi.stub $(srcdir)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop2)
	@$(TOUCH) $@

toolkit/mozapps/extensions/test/browser/misc: toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop2.zip.stub
toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop2.zip: toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop2.zip.stub ;
EXTRA_MDDEPEND_FILES += toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop2.zip.pp
toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop2.zip.stub: /worker/build/toolkit/mozapps/extensions/test/create_xpi.py $(srcdir)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop2
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/extensions/test/create_xpi.py main toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop2.zip toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop2.zip.pp toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop2.zip.stub $(srcdir)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop2)
	@$(TOUCH) $@

toolkit/mozapps/extensions/test/browser/misc: toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop_incompat.xpi.stub
toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop_incompat.xpi: toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop_incompat.xpi.stub ;
EXTRA_MDDEPEND_FILES += toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop_incompat.xpi.pp
toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop_incompat.xpi.stub: /worker/build/toolkit/mozapps/extensions/test/create_xpi.py $(srcdir)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop_incompat
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/extensions/test/create_xpi.py main toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop_incompat.xpi toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop_incompat.xpi.pp toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop_incompat.xpi.stub $(srcdir)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop_incompat)
	@$(TOUCH) $@

toolkit/mozapps/extensions/test/browser/misc: toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop_incompat.zip.stub
toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop_incompat.zip: toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop_incompat.zip.stub ;
EXTRA_MDDEPEND_FILES += toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop_incompat.zip.pp
toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop_incompat.zip.stub: /worker/build/toolkit/mozapps/extensions/test/create_xpi.py $(srcdir)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop_incompat
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/extensions/test/create_xpi.py main toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop_incompat.zip toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop_incompat.zip.pp toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_dragdrop_incompat.zip.stub $(srcdir)/toolkit/mozapps/extensions/test/browser/addons/browser_dragdrop_incompat)
	@$(TOUCH) $@

toolkit/mozapps/extensions/test/browser/misc: toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_installssl.xpi.stub
toolkit/mozapps/extensions/test/browser/addons/browser_installssl.xpi: toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_installssl.xpi.stub ;
EXTRA_MDDEPEND_FILES += toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_installssl.xpi.pp
toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_installssl.xpi.stub: /worker/build/toolkit/mozapps/extensions/test/create_xpi.py $(srcdir)/toolkit/mozapps/extensions/test/browser/addons/browser_installssl
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/extensions/test/create_xpi.py main toolkit/mozapps/extensions/test/browser/addons/browser_installssl.xpi toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_installssl.xpi.pp toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_installssl.xpi.stub $(srcdir)/toolkit/mozapps/extensions/test/browser/addons/browser_installssl)
	@$(TOUCH) $@

toolkit/mozapps/extensions/test/browser/misc: toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_installssl.zip.stub
toolkit/mozapps/extensions/test/browser/addons/browser_installssl.zip: toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_installssl.zip.stub ;
EXTRA_MDDEPEND_FILES += toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_installssl.zip.pp
toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_installssl.zip.stub: /worker/build/toolkit/mozapps/extensions/test/create_xpi.py $(srcdir)/toolkit/mozapps/extensions/test/browser/addons/browser_installssl
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/extensions/test/create_xpi.py main toolkit/mozapps/extensions/test/browser/addons/browser_installssl.zip toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_installssl.zip.pp toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_installssl.zip.stub $(srcdir)/toolkit/mozapps/extensions/test/browser/addons/browser_installssl)
	@$(TOUCH) $@

toolkit/mozapps/extensions/test/browser/misc: toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_theme.xpi.stub
toolkit/mozapps/extensions/test/browser/addons/browser_theme.xpi: toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_theme.xpi.stub ;
EXTRA_MDDEPEND_FILES += toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_theme.xpi.pp
toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_theme.xpi.stub: /worker/build/toolkit/mozapps/extensions/test/create_xpi.py $(srcdir)/toolkit/mozapps/extensions/test/browser/addons/browser_theme
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/extensions/test/create_xpi.py main toolkit/mozapps/extensions/test/browser/addons/browser_theme.xpi toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_theme.xpi.pp toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_theme.xpi.stub $(srcdir)/toolkit/mozapps/extensions/test/browser/addons/browser_theme)
	@$(TOUCH) $@

toolkit/mozapps/extensions/test/browser/misc: toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_theme.zip.stub
toolkit/mozapps/extensions/test/browser/addons/browser_theme.zip: toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_theme.zip.stub ;
EXTRA_MDDEPEND_FILES += toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_theme.zip.pp
toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_theme.zip.stub: /worker/build/toolkit/mozapps/extensions/test/create_xpi.py $(srcdir)/toolkit/mozapps/extensions/test/browser/addons/browser_theme
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/extensions/test/create_xpi.py main toolkit/mozapps/extensions/test/browser/addons/browser_theme.zip toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_theme.zip.pp toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/browser_theme.zip.stub $(srcdir)/toolkit/mozapps/extensions/test/browser/addons/browser_theme)
	@$(TOUCH) $@

toolkit/mozapps/extensions/test/browser/misc: toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/options_signed.xpi.stub
toolkit/mozapps/extensions/test/browser/addons/options_signed.xpi: toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/options_signed.xpi.stub ;
EXTRA_MDDEPEND_FILES += toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/options_signed.xpi.pp
toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/options_signed.xpi.stub: /worker/build/toolkit/mozapps/extensions/test/create_xpi.py $(srcdir)/toolkit/mozapps/extensions/test/browser/addons/options_signed
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/extensions/test/create_xpi.py main toolkit/mozapps/extensions/test/browser/addons/options_signed.xpi toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/options_signed.xpi.pp toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/options_signed.xpi.stub $(srcdir)/toolkit/mozapps/extensions/test/browser/addons/options_signed)
	@$(TOUCH) $@

toolkit/mozapps/extensions/test/browser/misc: toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/options_signed.zip.stub
toolkit/mozapps/extensions/test/browser/addons/options_signed.zip: toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/options_signed.zip.stub ;
EXTRA_MDDEPEND_FILES += toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/options_signed.zip.pp
toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/options_signed.zip.stub: /worker/build/toolkit/mozapps/extensions/test/create_xpi.py $(srcdir)/toolkit/mozapps/extensions/test/browser/addons/options_signed
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/extensions/test/create_xpi.py main toolkit/mozapps/extensions/test/browser/addons/options_signed.zip toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/options_signed.zip.pp toolkit/mozapps/extensions/test/browser/addons/$(MDDEPDIR)/options_signed.zip.stub $(srcdir)/toolkit/mozapps/extensions/test/browser/addons/options_signed)
	@$(TOUCH) $@

toolkit/mozapps/update/updater/export: toolkit/mozapps/update/updater/$(MDDEPDIR)/dep1Cert.h.stub
toolkit/mozapps/update/updater/dep1Cert.h: toolkit/mozapps/update/updater/$(MDDEPDIR)/dep1Cert.h.stub ;
EXTRA_MDDEPEND_FILES += toolkit/mozapps/update/updater/$(MDDEPDIR)/dep1Cert.h.pp
toolkit/mozapps/update/updater/$(MDDEPDIR)/dep1Cert.h.stub: /worker/build/toolkit/mozapps/update/updater/gen_cert_header.py $(srcdir)/toolkit/mozapps/update/updater/dep1.der
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/update/updater/gen_cert_header.py create_header toolkit/mozapps/update/updater/dep1Cert.h toolkit/mozapps/update/updater/$(MDDEPDIR)/dep1Cert.h.pp toolkit/mozapps/update/updater/$(MDDEPDIR)/dep1Cert.h.stub $(srcdir)/toolkit/mozapps/update/updater/dep1.der)
	@$(TOUCH) $@

toolkit/mozapps/update/updater/export: toolkit/mozapps/update/updater/$(MDDEPDIR)/dep2Cert.h.stub
toolkit/mozapps/update/updater/dep2Cert.h: toolkit/mozapps/update/updater/$(MDDEPDIR)/dep2Cert.h.stub ;
EXTRA_MDDEPEND_FILES += toolkit/mozapps/update/updater/$(MDDEPDIR)/dep2Cert.h.pp
toolkit/mozapps/update/updater/$(MDDEPDIR)/dep2Cert.h.stub: /worker/build/toolkit/mozapps/update/updater/gen_cert_header.py $(srcdir)/toolkit/mozapps/update/updater/dep2.der
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/update/updater/gen_cert_header.py create_header toolkit/mozapps/update/updater/dep2Cert.h toolkit/mozapps/update/updater/$(MDDEPDIR)/dep2Cert.h.pp toolkit/mozapps/update/updater/$(MDDEPDIR)/dep2Cert.h.stub $(srcdir)/toolkit/mozapps/update/updater/dep2.der)
	@$(TOUCH) $@

toolkit/mozapps/update/updater/export: toolkit/mozapps/update/updater/$(MDDEPDIR)/primaryCert.h.stub
toolkit/mozapps/update/updater/primaryCert.h: toolkit/mozapps/update/updater/$(MDDEPDIR)/primaryCert.h.stub ;
EXTRA_MDDEPEND_FILES += toolkit/mozapps/update/updater/$(MDDEPDIR)/primaryCert.h.pp
toolkit/mozapps/update/updater/$(MDDEPDIR)/primaryCert.h.stub: /worker/build/toolkit/mozapps/update/updater/gen_cert_header.py $(srcdir)/toolkit/mozapps/update/updater/release_primary.der
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/update/updater/gen_cert_header.py create_header toolkit/mozapps/update/updater/primaryCert.h toolkit/mozapps/update/updater/$(MDDEPDIR)/primaryCert.h.pp toolkit/mozapps/update/updater/$(MDDEPDIR)/primaryCert.h.stub $(srcdir)/toolkit/mozapps/update/updater/release_primary.der)
	@$(TOUCH) $@

toolkit/mozapps/update/updater/export: toolkit/mozapps/update/updater/$(MDDEPDIR)/secondaryCert.h.stub
toolkit/mozapps/update/updater/secondaryCert.h: toolkit/mozapps/update/updater/$(MDDEPDIR)/secondaryCert.h.stub ;
EXTRA_MDDEPEND_FILES += toolkit/mozapps/update/updater/$(MDDEPDIR)/secondaryCert.h.pp
toolkit/mozapps/update/updater/$(MDDEPDIR)/secondaryCert.h.stub: /worker/build/toolkit/mozapps/update/updater/gen_cert_header.py $(srcdir)/toolkit/mozapps/update/updater/release_secondary.der
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/update/updater/gen_cert_header.py create_header toolkit/mozapps/update/updater/secondaryCert.h toolkit/mozapps/update/updater/$(MDDEPDIR)/secondaryCert.h.pp toolkit/mozapps/update/updater/$(MDDEPDIR)/secondaryCert.h.stub $(srcdir)/toolkit/mozapps/update/updater/release_secondary.der)
	@$(TOUCH) $@

toolkit/mozapps/update/updater/export: toolkit/mozapps/update/updater/$(MDDEPDIR)/xpcshellCert.h.stub
toolkit/mozapps/update/updater/xpcshellCert.h: toolkit/mozapps/update/updater/$(MDDEPDIR)/xpcshellCert.h.stub ;
EXTRA_MDDEPEND_FILES += toolkit/mozapps/update/updater/$(MDDEPDIR)/xpcshellCert.h.pp
toolkit/mozapps/update/updater/$(MDDEPDIR)/xpcshellCert.h.stub: /worker/build/toolkit/mozapps/update/updater/gen_cert_header.py $(srcdir)/toolkit/mozapps/update/updater/xpcshellCertificate.der
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/mozapps/update/updater/gen_cert_header.py create_header toolkit/mozapps/update/updater/xpcshellCert.h toolkit/mozapps/update/updater/$(MDDEPDIR)/xpcshellCert.h.pp toolkit/mozapps/update/updater/$(MDDEPDIR)/xpcshellCert.h.stub $(srcdir)/toolkit/mozapps/update/updater/xpcshellCertificate.der)
	@$(TOUCH) $@

devtools/client/debugger/src/misc: devtools/client/debugger/src/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/node.stub: devtools/client/debugger/src/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/main.js: devtools/client/debugger/src/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/vendors.js: devtools/client/debugger/src/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/main.js $(srcdir)/devtools/client/debugger/src/vendors.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/node.stub devtools/client/debugger/src/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/main.js $(srcdir)/devtools/client/debugger/src/vendors.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src)
	@$(TOUCH) $@

devtools/client/debugger/src/actions/misc: devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/actions/node.stub: devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/event-listeners.js: devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/exceptions.js: devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/expressions.js: devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/file-search.js: devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/index.js: devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/navigation.js: devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/preview.js: devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/project-text-search.js: devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/quick-open.js: devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/source-actors.js: devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/source-tree.js: devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/tabs.js: devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/toolbox.js: devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/threads.js: devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/ui.js: devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/actions/event-listeners.js $(srcdir)/devtools/client/debugger/src/actions/exceptions.js $(srcdir)/devtools/client/debugger/src/actions/expressions.js $(srcdir)/devtools/client/debugger/src/actions/file-search.js $(srcdir)/devtools/client/debugger/src/actions/index.js $(srcdir)/devtools/client/debugger/src/actions/navigation.js $(srcdir)/devtools/client/debugger/src/actions/preview.js $(srcdir)/devtools/client/debugger/src/actions/project-text-search.js $(srcdir)/devtools/client/debugger/src/actions/quick-open.js $(srcdir)/devtools/client/debugger/src/actions/source-actors.js $(srcdir)/devtools/client/debugger/src/actions/source-tree.js $(srcdir)/devtools/client/debugger/src/actions/tabs.js $(srcdir)/devtools/client/debugger/src/actions/toolbox.js $(srcdir)/devtools/client/debugger/src/actions/threads.js $(srcdir)/devtools/client/debugger/src/actions/ui.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/actions/node.stub devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/actions/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/actions/event-listeners.js $(srcdir)/devtools/client/debugger/src/actions/exceptions.js $(srcdir)/devtools/client/debugger/src/actions/expressions.js $(srcdir)/devtools/client/debugger/src/actions/file-search.js $(srcdir)/devtools/client/debugger/src/actions/index.js $(srcdir)/devtools/client/debugger/src/actions/navigation.js $(srcdir)/devtools/client/debugger/src/actions/preview.js $(srcdir)/devtools/client/debugger/src/actions/project-text-search.js $(srcdir)/devtools/client/debugger/src/actions/quick-open.js $(srcdir)/devtools/client/debugger/src/actions/source-actors.js $(srcdir)/devtools/client/debugger/src/actions/source-tree.js $(srcdir)/devtools/client/debugger/src/actions/tabs.js $(srcdir)/devtools/client/debugger/src/actions/toolbox.js $(srcdir)/devtools/client/debugger/src/actions/threads.js $(srcdir)/devtools/client/debugger/src/actions/ui.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions)
	@$(TOUCH) $@

devtools/client/debugger/src/actions/ast/misc: devtools/client/debugger/src/actions/ast/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/actions/ast/node.stub: devtools/client/debugger/src/actions/ast/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/ast/index.js: devtools/client/debugger/src/actions/ast/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/ast/setInScopeLines.js: devtools/client/debugger/src/actions/ast/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/actions/ast/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/actions/ast/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/actions/ast/index.js $(srcdir)/devtools/client/debugger/src/actions/ast/setInScopeLines.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/actions/ast/node.stub devtools/client/debugger/src/actions/ast/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/actions/ast/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/actions/ast/index.js $(srcdir)/devtools/client/debugger/src/actions/ast/setInScopeLines.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/ast)
	@$(TOUCH) $@

devtools/client/debugger/src/actions/breakpoints/misc: devtools/client/debugger/src/actions/breakpoints/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/actions/breakpoints/node.stub: devtools/client/debugger/src/actions/breakpoints/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/breakpoints/breakpointPositions.js: devtools/client/debugger/src/actions/breakpoints/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/breakpoints/index.js: devtools/client/debugger/src/actions/breakpoints/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/breakpoints/modify.js: devtools/client/debugger/src/actions/breakpoints/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/breakpoints/remapLocations.js: devtools/client/debugger/src/actions/breakpoints/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/breakpoints/syncBreakpoint.js: devtools/client/debugger/src/actions/breakpoints/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/actions/breakpoints/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/actions/breakpoints/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/actions/breakpoints/breakpointPositions.js $(srcdir)/devtools/client/debugger/src/actions/breakpoints/index.js $(srcdir)/devtools/client/debugger/src/actions/breakpoints/modify.js $(srcdir)/devtools/client/debugger/src/actions/breakpoints/remapLocations.js $(srcdir)/devtools/client/debugger/src/actions/breakpoints/syncBreakpoint.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/actions/breakpoints/node.stub devtools/client/debugger/src/actions/breakpoints/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/actions/breakpoints/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/actions/breakpoints/breakpointPositions.js $(srcdir)/devtools/client/debugger/src/actions/breakpoints/index.js $(srcdir)/devtools/client/debugger/src/actions/breakpoints/modify.js $(srcdir)/devtools/client/debugger/src/actions/breakpoints/remapLocations.js $(srcdir)/devtools/client/debugger/src/actions/breakpoints/syncBreakpoint.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/breakpoints)
	@$(TOUCH) $@

devtools/client/debugger/src/actions/pause/misc: devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/actions/pause/node.stub: devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/breakOnNext.js: devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/commands.js: devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/continueToHere.js: devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/expandScopes.js: devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/fetchFrames.js: devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/fetchScopes.js: devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/index.js: devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/inlinePreview.js: devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/mapDisplayNames.js: devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/mapFrames.js: devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/mapScopes.js: devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/paused.js: devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/pauseOnExceptions.js: devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/previewPausedLocation.js: devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/resumed.js: devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/selectFrame.js: devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/highlightCalls.js: devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause/skipPausing.js: devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/actions/pause/breakOnNext.js $(srcdir)/devtools/client/debugger/src/actions/pause/commands.js $(srcdir)/devtools/client/debugger/src/actions/pause/continueToHere.js $(srcdir)/devtools/client/debugger/src/actions/pause/expandScopes.js $(srcdir)/devtools/client/debugger/src/actions/pause/fetchFrames.js $(srcdir)/devtools/client/debugger/src/actions/pause/fetchScopes.js $(srcdir)/devtools/client/debugger/src/actions/pause/index.js $(srcdir)/devtools/client/debugger/src/actions/pause/inlinePreview.js $(srcdir)/devtools/client/debugger/src/actions/pause/mapDisplayNames.js $(srcdir)/devtools/client/debugger/src/actions/pause/mapFrames.js $(srcdir)/devtools/client/debugger/src/actions/pause/mapScopes.js $(srcdir)/devtools/client/debugger/src/actions/pause/paused.js $(srcdir)/devtools/client/debugger/src/actions/pause/pauseOnExceptions.js $(srcdir)/devtools/client/debugger/src/actions/pause/previewPausedLocation.js $(srcdir)/devtools/client/debugger/src/actions/pause/resumed.js $(srcdir)/devtools/client/debugger/src/actions/pause/selectFrame.js $(srcdir)/devtools/client/debugger/src/actions/pause/highlightCalls.js $(srcdir)/devtools/client/debugger/src/actions/pause/skipPausing.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/actions/pause/node.stub devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/actions/pause/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/actions/pause/breakOnNext.js $(srcdir)/devtools/client/debugger/src/actions/pause/commands.js $(srcdir)/devtools/client/debugger/src/actions/pause/continueToHere.js $(srcdir)/devtools/client/debugger/src/actions/pause/expandScopes.js $(srcdir)/devtools/client/debugger/src/actions/pause/fetchFrames.js $(srcdir)/devtools/client/debugger/src/actions/pause/fetchScopes.js $(srcdir)/devtools/client/debugger/src/actions/pause/index.js $(srcdir)/devtools/client/debugger/src/actions/pause/inlinePreview.js $(srcdir)/devtools/client/debugger/src/actions/pause/mapDisplayNames.js $(srcdir)/devtools/client/debugger/src/actions/pause/mapFrames.js $(srcdir)/devtools/client/debugger/src/actions/pause/mapScopes.js $(srcdir)/devtools/client/debugger/src/actions/pause/paused.js $(srcdir)/devtools/client/debugger/src/actions/pause/pauseOnExceptions.js $(srcdir)/devtools/client/debugger/src/actions/pause/previewPausedLocation.js $(srcdir)/devtools/client/debugger/src/actions/pause/resumed.js $(srcdir)/devtools/client/debugger/src/actions/pause/selectFrame.js $(srcdir)/devtools/client/debugger/src/actions/pause/highlightCalls.js $(srcdir)/devtools/client/debugger/src/actions/pause/skipPausing.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/pause)
	@$(TOUCH) $@

devtools/client/debugger/src/actions/sources/misc: devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/actions/sources/node.stub: devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/sources/blackbox.js: devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/sources/breakableLines.js: devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/sources/index.js: devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/sources/loadSourceText.js: devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/sources/newSources.js: devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/sources/prettyPrint.js: devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/sources/select.js: devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/sources/symbols.js: devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/actions/sources/blackbox.js $(srcdir)/devtools/client/debugger/src/actions/sources/breakableLines.js $(srcdir)/devtools/client/debugger/src/actions/sources/index.js $(srcdir)/devtools/client/debugger/src/actions/sources/loadSourceText.js $(srcdir)/devtools/client/debugger/src/actions/sources/newSources.js $(srcdir)/devtools/client/debugger/src/actions/sources/prettyPrint.js $(srcdir)/devtools/client/debugger/src/actions/sources/select.js $(srcdir)/devtools/client/debugger/src/actions/sources/symbols.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/actions/sources/node.stub devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/actions/sources/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/actions/sources/blackbox.js $(srcdir)/devtools/client/debugger/src/actions/sources/breakableLines.js $(srcdir)/devtools/client/debugger/src/actions/sources/index.js $(srcdir)/devtools/client/debugger/src/actions/sources/loadSourceText.js $(srcdir)/devtools/client/debugger/src/actions/sources/newSources.js $(srcdir)/devtools/client/debugger/src/actions/sources/prettyPrint.js $(srcdir)/devtools/client/debugger/src/actions/sources/select.js $(srcdir)/devtools/client/debugger/src/actions/sources/symbols.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/sources)
	@$(TOUCH) $@

devtools/client/debugger/src/actions/utils/misc: devtools/client/debugger/src/actions/utils/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/actions/utils/node.stub: devtools/client/debugger/src/actions/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/utils/create-store.js: devtools/client/debugger/src/actions/utils/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/actions/utils/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/actions/utils/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/actions/utils/create-store.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/actions/utils/node.stub devtools/client/debugger/src/actions/utils/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/actions/utils/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/actions/utils/create-store.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/utils)
	@$(TOUCH) $@

devtools/client/debugger/src/actions/utils/middleware/misc: devtools/client/debugger/src/actions/utils/middleware/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/actions/utils/middleware/node.stub: devtools/client/debugger/src/actions/utils/middleware/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/utils/middleware/context.js: devtools/client/debugger/src/actions/utils/middleware/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/utils/middleware/log.js: devtools/client/debugger/src/actions/utils/middleware/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/utils/middleware/promise.js: devtools/client/debugger/src/actions/utils/middleware/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/utils/middleware/thunk.js: devtools/client/debugger/src/actions/utils/middleware/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/utils/middleware/timing.js: devtools/client/debugger/src/actions/utils/middleware/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/utils/middleware/wait-service.js: devtools/client/debugger/src/actions/utils/middleware/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/actions/utils/middleware/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/actions/utils/middleware/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/actions/utils/middleware/context.js $(srcdir)/devtools/client/debugger/src/actions/utils/middleware/log.js $(srcdir)/devtools/client/debugger/src/actions/utils/middleware/promise.js $(srcdir)/devtools/client/debugger/src/actions/utils/middleware/thunk.js $(srcdir)/devtools/client/debugger/src/actions/utils/middleware/timing.js $(srcdir)/devtools/client/debugger/src/actions/utils/middleware/wait-service.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/actions/utils/middleware/node.stub devtools/client/debugger/src/actions/utils/middleware/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/actions/utils/middleware/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/actions/utils/middleware/context.js $(srcdir)/devtools/client/debugger/src/actions/utils/middleware/log.js $(srcdir)/devtools/client/debugger/src/actions/utils/middleware/promise.js $(srcdir)/devtools/client/debugger/src/actions/utils/middleware/thunk.js $(srcdir)/devtools/client/debugger/src/actions/utils/middleware/timing.js $(srcdir)/devtools/client/debugger/src/actions/utils/middleware/wait-service.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/actions/utils/middleware)
	@$(TOUCH) $@

devtools/client/debugger/src/client/misc: devtools/client/debugger/src/client/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/client/node.stub: devtools/client/debugger/src/client/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/client/firefox.js: devtools/client/debugger/src/client/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/client/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/client/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/client/firefox.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/client/node.stub devtools/client/debugger/src/client/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/client/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/client/firefox.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/client)
	@$(TOUCH) $@

devtools/client/debugger/src/client/firefox/misc: devtools/client/debugger/src/client/firefox/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/client/firefox/node.stub: devtools/client/debugger/src/client/firefox/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/client/firefox/commands.js: devtools/client/debugger/src/client/firefox/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/client/firefox/create.js: devtools/client/debugger/src/client/firefox/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/client/firefox/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/client/firefox/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/client/firefox/commands.js $(srcdir)/devtools/client/debugger/src/client/firefox/create.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/client/firefox/node.stub devtools/client/debugger/src/client/firefox/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/client/firefox/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/client/firefox/commands.js $(srcdir)/devtools/client/debugger/src/client/firefox/create.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/client/firefox)
	@$(TOUCH) $@

devtools/client/debugger/src/components/misc: devtools/client/debugger/src/components/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/components/node.stub: devtools/client/debugger/src/components/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/A11yIntention.js: devtools/client/debugger/src/components/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/App.js: devtools/client/debugger/src/components/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/ProjectSearch.js: devtools/client/debugger/src/components/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/QuickOpenModal.js: devtools/client/debugger/src/components/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/ShortcutsModal.js: devtools/client/debugger/src/components/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/WelcomeBox.js: devtools/client/debugger/src/components/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/components/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/components/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/components/A11yIntention.js $(srcdir)/devtools/client/debugger/src/components/App.js $(srcdir)/devtools/client/debugger/src/components/ProjectSearch.js $(srcdir)/devtools/client/debugger/src/components/QuickOpenModal.js $(srcdir)/devtools/client/debugger/src/components/ShortcutsModal.js $(srcdir)/devtools/client/debugger/src/components/WelcomeBox.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/components/node.stub devtools/client/debugger/src/components/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/components/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/components/A11yIntention.js $(srcdir)/devtools/client/debugger/src/components/App.js $(srcdir)/devtools/client/debugger/src/components/ProjectSearch.js $(srcdir)/devtools/client/debugger/src/components/QuickOpenModal.js $(srcdir)/devtools/client/debugger/src/components/ShortcutsModal.js $(srcdir)/devtools/client/debugger/src/components/WelcomeBox.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components)
	@$(TOUCH) $@

devtools/client/debugger/src/components/Editor/misc: devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/components/Editor/node.stub: devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/Breakpoint.js: devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/Breakpoints.js: devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/ColumnBreakpoint.js: devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/ColumnBreakpoints.js: devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/ConditionalPanel.js: devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/DebugLine.js: devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/EditorMenu.js: devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/EmptyLines.js: devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/Exception.js: devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/Exceptions.js: devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/Footer.js: devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/HighlightCalls.js: devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/HighlightLine.js: devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/HighlightLines.js: devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/index.js: devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/InlinePreview.js: devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/InlinePreviewRow.js: devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/InlinePreviews.js: devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/SearchBar.js: devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/Tab.js: devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/Tabs.js: devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/components/Editor/Breakpoint.js $(srcdir)/devtools/client/debugger/src/components/Editor/Breakpoints.js $(srcdir)/devtools/client/debugger/src/components/Editor/ColumnBreakpoint.js $(srcdir)/devtools/client/debugger/src/components/Editor/ColumnBreakpoints.js $(srcdir)/devtools/client/debugger/src/components/Editor/ConditionalPanel.js $(srcdir)/devtools/client/debugger/src/components/Editor/DebugLine.js $(srcdir)/devtools/client/debugger/src/components/Editor/EditorMenu.js $(srcdir)/devtools/client/debugger/src/components/Editor/EmptyLines.js $(srcdir)/devtools/client/debugger/src/components/Editor/Exception.js $(srcdir)/devtools/client/debugger/src/components/Editor/Exceptions.js $(srcdir)/devtools/client/debugger/src/components/Editor/Footer.js $(srcdir)/devtools/client/debugger/src/components/Editor/HighlightCalls.js $(srcdir)/devtools/client/debugger/src/components/Editor/HighlightLine.js $(srcdir)/devtools/client/debugger/src/components/Editor/HighlightLines.js $(srcdir)/devtools/client/debugger/src/components/Editor/index.js $(srcdir)/devtools/client/debugger/src/components/Editor/InlinePreview.js $(srcdir)/devtools/client/debugger/src/components/Editor/InlinePreviewRow.js $(srcdir)/devtools/client/debugger/src/components/Editor/InlinePreviews.js $(srcdir)/devtools/client/debugger/src/components/Editor/SearchBar.js $(srcdir)/devtools/client/debugger/src/components/Editor/Tab.js $(srcdir)/devtools/client/debugger/src/components/Editor/Tabs.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/components/Editor/node.stub devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/components/Editor/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/components/Editor/Breakpoint.js $(srcdir)/devtools/client/debugger/src/components/Editor/Breakpoints.js $(srcdir)/devtools/client/debugger/src/components/Editor/ColumnBreakpoint.js $(srcdir)/devtools/client/debugger/src/components/Editor/ColumnBreakpoints.js $(srcdir)/devtools/client/debugger/src/components/Editor/ConditionalPanel.js $(srcdir)/devtools/client/debugger/src/components/Editor/DebugLine.js $(srcdir)/devtools/client/debugger/src/components/Editor/EditorMenu.js $(srcdir)/devtools/client/debugger/src/components/Editor/EmptyLines.js $(srcdir)/devtools/client/debugger/src/components/Editor/Exception.js $(srcdir)/devtools/client/debugger/src/components/Editor/Exceptions.js $(srcdir)/devtools/client/debugger/src/components/Editor/Footer.js $(srcdir)/devtools/client/debugger/src/components/Editor/HighlightCalls.js $(srcdir)/devtools/client/debugger/src/components/Editor/HighlightLine.js $(srcdir)/devtools/client/debugger/src/components/Editor/HighlightLines.js $(srcdir)/devtools/client/debugger/src/components/Editor/index.js $(srcdir)/devtools/client/debugger/src/components/Editor/InlinePreview.js $(srcdir)/devtools/client/debugger/src/components/Editor/InlinePreviewRow.js $(srcdir)/devtools/client/debugger/src/components/Editor/InlinePreviews.js $(srcdir)/devtools/client/debugger/src/components/Editor/SearchBar.js $(srcdir)/devtools/client/debugger/src/components/Editor/Tab.js $(srcdir)/devtools/client/debugger/src/components/Editor/Tabs.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor)
	@$(TOUCH) $@

devtools/client/debugger/src/components/Editor/menus/misc: devtools/client/debugger/src/components/Editor/menus/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/components/Editor/menus/node.stub: devtools/client/debugger/src/components/Editor/menus/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/menus/breakpoints.js: devtools/client/debugger/src/components/Editor/menus/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/menus/editor.js: devtools/client/debugger/src/components/Editor/menus/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/menus/source.js: devtools/client/debugger/src/components/Editor/menus/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/components/Editor/menus/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/components/Editor/menus/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/components/Editor/menus/breakpoints.js $(srcdir)/devtools/client/debugger/src/components/Editor/menus/editor.js $(srcdir)/devtools/client/debugger/src/components/Editor/menus/source.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/components/Editor/menus/node.stub devtools/client/debugger/src/components/Editor/menus/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/components/Editor/menus/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/components/Editor/menus/breakpoints.js $(srcdir)/devtools/client/debugger/src/components/Editor/menus/editor.js $(srcdir)/devtools/client/debugger/src/components/Editor/menus/source.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/menus)
	@$(TOUCH) $@

devtools/client/debugger/src/components/Editor/Preview/misc: devtools/client/debugger/src/components/Editor/Preview/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/components/Editor/Preview/node.stub: devtools/client/debugger/src/components/Editor/Preview/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/Preview/ExceptionPopup.js: devtools/client/debugger/src/components/Editor/Preview/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/Preview/index.js: devtools/client/debugger/src/components/Editor/Preview/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/Preview/Popup.js: devtools/client/debugger/src/components/Editor/Preview/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/components/Editor/Preview/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/components/Editor/Preview/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/components/Editor/Preview/ExceptionPopup.js $(srcdir)/devtools/client/debugger/src/components/Editor/Preview/index.js $(srcdir)/devtools/client/debugger/src/components/Editor/Preview/Popup.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/components/Editor/Preview/node.stub devtools/client/debugger/src/components/Editor/Preview/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/components/Editor/Preview/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/components/Editor/Preview/ExceptionPopup.js $(srcdir)/devtools/client/debugger/src/components/Editor/Preview/index.js $(srcdir)/devtools/client/debugger/src/components/Editor/Preview/Popup.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/Editor/Preview)
	@$(TOUCH) $@

devtools/client/debugger/src/components/PrimaryPanes/misc: devtools/client/debugger/src/components/PrimaryPanes/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/components/PrimaryPanes/node.stub: devtools/client/debugger/src/components/PrimaryPanes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/PrimaryPanes/index.js: devtools/client/debugger/src/components/PrimaryPanes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/PrimaryPanes/Outline.js: devtools/client/debugger/src/components/PrimaryPanes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/PrimaryPanes/OutlineFilter.js: devtools/client/debugger/src/components/PrimaryPanes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/PrimaryPanes/SourcesTree.js: devtools/client/debugger/src/components/PrimaryPanes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/PrimaryPanes/SourcesTreeItem.js: devtools/client/debugger/src/components/PrimaryPanes/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/components/PrimaryPanes/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/components/PrimaryPanes/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/components/PrimaryPanes/index.js $(srcdir)/devtools/client/debugger/src/components/PrimaryPanes/Outline.js $(srcdir)/devtools/client/debugger/src/components/PrimaryPanes/OutlineFilter.js $(srcdir)/devtools/client/debugger/src/components/PrimaryPanes/SourcesTree.js $(srcdir)/devtools/client/debugger/src/components/PrimaryPanes/SourcesTreeItem.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/components/PrimaryPanes/node.stub devtools/client/debugger/src/components/PrimaryPanes/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/components/PrimaryPanes/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/components/PrimaryPanes/index.js $(srcdir)/devtools/client/debugger/src/components/PrimaryPanes/Outline.js $(srcdir)/devtools/client/debugger/src/components/PrimaryPanes/OutlineFilter.js $(srcdir)/devtools/client/debugger/src/components/PrimaryPanes/SourcesTree.js $(srcdir)/devtools/client/debugger/src/components/PrimaryPanes/SourcesTreeItem.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/PrimaryPanes)
	@$(TOUCH) $@

devtools/client/debugger/src/components/SecondaryPanes/misc: devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/components/SecondaryPanes/node.stub: devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/CommandBar.js: devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/DOMMutationBreakpoints.js: devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/EventListeners.js: devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Expressions.js: devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/index.js: devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Scopes.js: devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Thread.js: devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Threads.js: devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/WhyPaused.js: devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/XHRBreakpoints.js: devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/CommandBar.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/DOMMutationBreakpoints.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/EventListeners.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Expressions.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/index.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Scopes.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Thread.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Threads.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/WhyPaused.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/XHRBreakpoints.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/components/SecondaryPanes/node.stub devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/components/SecondaryPanes/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/CommandBar.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/DOMMutationBreakpoints.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/EventListeners.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Expressions.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/index.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Scopes.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Thread.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Threads.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/WhyPaused.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/XHRBreakpoints.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes)
	@$(TOUCH) $@

devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/misc: devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/node.stub: devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/Breakpoint.js: devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/BreakpointHeading.js: devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/BreakpointHeadingsContextMenu.js: devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/BreakpointsContextMenu.js: devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/ExceptionOption.js: devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/index.js: devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/Breakpoint.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/BreakpointHeading.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/BreakpointHeadingsContextMenu.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/BreakpointsContextMenu.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/ExceptionOption.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/index.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/node.stub devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/Breakpoint.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/BreakpointHeading.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/BreakpointHeadingsContextMenu.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/BreakpointsContextMenu.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/ExceptionOption.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints/index.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Breakpoints)
	@$(TOUCH) $@

devtools/client/debugger/src/components/SecondaryPanes/Frames/misc: devtools/client/debugger/src/components/SecondaryPanes/Frames/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/components/SecondaryPanes/Frames/node.stub: devtools/client/debugger/src/components/SecondaryPanes/Frames/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Frames/Frame.js: devtools/client/debugger/src/components/SecondaryPanes/Frames/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Frames/FrameIndent.js: devtools/client/debugger/src/components/SecondaryPanes/Frames/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Frames/FrameMenu.js: devtools/client/debugger/src/components/SecondaryPanes/Frames/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Frames/Group.js: devtools/client/debugger/src/components/SecondaryPanes/Frames/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Frames/index.js: devtools/client/debugger/src/components/SecondaryPanes/Frames/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/components/SecondaryPanes/Frames/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/components/SecondaryPanes/Frames/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Frames/Frame.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Frames/FrameIndent.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Frames/FrameMenu.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Frames/Group.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Frames/index.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/components/SecondaryPanes/Frames/node.stub devtools/client/debugger/src/components/SecondaryPanes/Frames/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/components/SecondaryPanes/Frames/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Frames/Frame.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Frames/FrameIndent.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Frames/FrameMenu.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Frames/Group.js $(srcdir)/devtools/client/debugger/src/components/SecondaryPanes/Frames/index.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/SecondaryPanes/Frames)
	@$(TOUCH) $@

devtools/client/debugger/src/components/shared/misc: devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/components/shared/node.stub: devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/AccessibleImage.js: devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/Accordion.js: devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/Badge.js: devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/BracketArrow.js: devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/Dropdown.js: devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/ManagedTree.js: devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/Modal.js: devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/Popover.js: devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/PreviewFunction.js: devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/ResultList.js: devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/SearchInput.js: devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/SourceIcon.js: devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/SmartGap.js: devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/components/shared/AccessibleImage.js $(srcdir)/devtools/client/debugger/src/components/shared/Accordion.js $(srcdir)/devtools/client/debugger/src/components/shared/Badge.js $(srcdir)/devtools/client/debugger/src/components/shared/BracketArrow.js $(srcdir)/devtools/client/debugger/src/components/shared/Dropdown.js $(srcdir)/devtools/client/debugger/src/components/shared/ManagedTree.js $(srcdir)/devtools/client/debugger/src/components/shared/Modal.js $(srcdir)/devtools/client/debugger/src/components/shared/Popover.js $(srcdir)/devtools/client/debugger/src/components/shared/PreviewFunction.js $(srcdir)/devtools/client/debugger/src/components/shared/ResultList.js $(srcdir)/devtools/client/debugger/src/components/shared/SearchInput.js $(srcdir)/devtools/client/debugger/src/components/shared/SourceIcon.js $(srcdir)/devtools/client/debugger/src/components/shared/SmartGap.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/components/shared/node.stub devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/components/shared/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/components/shared/AccessibleImage.js $(srcdir)/devtools/client/debugger/src/components/shared/Accordion.js $(srcdir)/devtools/client/debugger/src/components/shared/Badge.js $(srcdir)/devtools/client/debugger/src/components/shared/BracketArrow.js $(srcdir)/devtools/client/debugger/src/components/shared/Dropdown.js $(srcdir)/devtools/client/debugger/src/components/shared/ManagedTree.js $(srcdir)/devtools/client/debugger/src/components/shared/Modal.js $(srcdir)/devtools/client/debugger/src/components/shared/Popover.js $(srcdir)/devtools/client/debugger/src/components/shared/PreviewFunction.js $(srcdir)/devtools/client/debugger/src/components/shared/ResultList.js $(srcdir)/devtools/client/debugger/src/components/shared/SearchInput.js $(srcdir)/devtools/client/debugger/src/components/shared/SourceIcon.js $(srcdir)/devtools/client/debugger/src/components/shared/SmartGap.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared)
	@$(TOUCH) $@

devtools/client/debugger/src/components/shared/Button/misc: devtools/client/debugger/src/components/shared/Button/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/components/shared/Button/node.stub: devtools/client/debugger/src/components/shared/Button/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/Button/CloseButton.js: devtools/client/debugger/src/components/shared/Button/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/Button/CommandBarButton.js: devtools/client/debugger/src/components/shared/Button/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/Button/index.js: devtools/client/debugger/src/components/shared/Button/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/Button/PaneToggleButton.js: devtools/client/debugger/src/components/shared/Button/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/components/shared/Button/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/components/shared/Button/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/components/shared/Button/CloseButton.js $(srcdir)/devtools/client/debugger/src/components/shared/Button/CommandBarButton.js $(srcdir)/devtools/client/debugger/src/components/shared/Button/index.js $(srcdir)/devtools/client/debugger/src/components/shared/Button/PaneToggleButton.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/components/shared/Button/node.stub devtools/client/debugger/src/components/shared/Button/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/components/shared/Button/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/components/shared/Button/CloseButton.js $(srcdir)/devtools/client/debugger/src/components/shared/Button/CommandBarButton.js $(srcdir)/devtools/client/debugger/src/components/shared/Button/index.js $(srcdir)/devtools/client/debugger/src/components/shared/Button/PaneToggleButton.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/Button)
	@$(TOUCH) $@

devtools/client/debugger/src/components/shared/Button/styles/misc: devtools/client/debugger/src/components/shared/Button/styles/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/components/shared/Button/styles/node.stub: devtools/client/debugger/src/components/shared/Button/styles/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/components/shared/Button/styles/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/components/shared/Button/styles/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/components/shared/Button/styles/node.stub devtools/client/debugger/src/components/shared/Button/styles/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/components/shared/Button/styles/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/components/shared/Button/styles)
	@$(TOUCH) $@

devtools/client/debugger/src/context-menu/misc: devtools/client/debugger/src/context-menu/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/context-menu/node.stub: devtools/client/debugger/src/context-menu/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/context-menu/menu.js: devtools/client/debugger/src/context-menu/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/context-menu/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/context-menu/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/context-menu/menu.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/context-menu/node.stub devtools/client/debugger/src/context-menu/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/context-menu/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/context-menu/menu.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/context-menu)
	@$(TOUCH) $@

devtools/client/debugger/src/reducers/misc: devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/reducers/node.stub: devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/ast.js: devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/async-requests.js: devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/breakpoints.js: devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/event-listeners.js: devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/exceptions.js: devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/expressions.js: devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/file-search.js: devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/index.js: devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/pause.js: devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/pending-breakpoints.js: devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/preview.js: devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/project-text-search.js: devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/quick-open.js: devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/source-actors.js: devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/source-tree.js: devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/sources.js: devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/tabs.js: devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/threads.js: devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers/ui.js: devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/reducers/ast.js $(srcdir)/devtools/client/debugger/src/reducers/async-requests.js $(srcdir)/devtools/client/debugger/src/reducers/breakpoints.js $(srcdir)/devtools/client/debugger/src/reducers/event-listeners.js $(srcdir)/devtools/client/debugger/src/reducers/exceptions.js $(srcdir)/devtools/client/debugger/src/reducers/expressions.js $(srcdir)/devtools/client/debugger/src/reducers/file-search.js $(srcdir)/devtools/client/debugger/src/reducers/index.js $(srcdir)/devtools/client/debugger/src/reducers/pause.js $(srcdir)/devtools/client/debugger/src/reducers/pending-breakpoints.js $(srcdir)/devtools/client/debugger/src/reducers/preview.js $(srcdir)/devtools/client/debugger/src/reducers/project-text-search.js $(srcdir)/devtools/client/debugger/src/reducers/quick-open.js $(srcdir)/devtools/client/debugger/src/reducers/source-actors.js $(srcdir)/devtools/client/debugger/src/reducers/source-tree.js $(srcdir)/devtools/client/debugger/src/reducers/sources.js $(srcdir)/devtools/client/debugger/src/reducers/tabs.js $(srcdir)/devtools/client/debugger/src/reducers/threads.js $(srcdir)/devtools/client/debugger/src/reducers/ui.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/reducers/node.stub devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/reducers/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/reducers/ast.js $(srcdir)/devtools/client/debugger/src/reducers/async-requests.js $(srcdir)/devtools/client/debugger/src/reducers/breakpoints.js $(srcdir)/devtools/client/debugger/src/reducers/event-listeners.js $(srcdir)/devtools/client/debugger/src/reducers/exceptions.js $(srcdir)/devtools/client/debugger/src/reducers/expressions.js $(srcdir)/devtools/client/debugger/src/reducers/file-search.js $(srcdir)/devtools/client/debugger/src/reducers/index.js $(srcdir)/devtools/client/debugger/src/reducers/pause.js $(srcdir)/devtools/client/debugger/src/reducers/pending-breakpoints.js $(srcdir)/devtools/client/debugger/src/reducers/preview.js $(srcdir)/devtools/client/debugger/src/reducers/project-text-search.js $(srcdir)/devtools/client/debugger/src/reducers/quick-open.js $(srcdir)/devtools/client/debugger/src/reducers/source-actors.js $(srcdir)/devtools/client/debugger/src/reducers/source-tree.js $(srcdir)/devtools/client/debugger/src/reducers/sources.js $(srcdir)/devtools/client/debugger/src/reducers/tabs.js $(srcdir)/devtools/client/debugger/src/reducers/threads.js $(srcdir)/devtools/client/debugger/src/reducers/ui.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/reducers)
	@$(TOUCH) $@

devtools/client/debugger/src/selectors/misc: devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/selectors/node.stub: devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/selectors/breakpointAtLocation.js: devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/selectors/breakpoints.js: devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/selectors/breakpointSources.js: devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/selectors/getCallStackFrames.js: devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/selectors/inComponent.js: devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/selectors/index.js: devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/selectors/isLineInScope.js: devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/selectors/isSelectedFrameVisible.js: devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/selectors/pause.js: devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/selectors/visibleBreakpoints.js: devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/selectors/visibleColumnBreakpoints.js: devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/selectors/breakpointAtLocation.js $(srcdir)/devtools/client/debugger/src/selectors/breakpoints.js $(srcdir)/devtools/client/debugger/src/selectors/breakpointSources.js $(srcdir)/devtools/client/debugger/src/selectors/getCallStackFrames.js $(srcdir)/devtools/client/debugger/src/selectors/inComponent.js $(srcdir)/devtools/client/debugger/src/selectors/index.js $(srcdir)/devtools/client/debugger/src/selectors/isLineInScope.js $(srcdir)/devtools/client/debugger/src/selectors/isSelectedFrameVisible.js $(srcdir)/devtools/client/debugger/src/selectors/pause.js $(srcdir)/devtools/client/debugger/src/selectors/visibleBreakpoints.js $(srcdir)/devtools/client/debugger/src/selectors/visibleColumnBreakpoints.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/selectors/node.stub devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/selectors/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/selectors/breakpointAtLocation.js $(srcdir)/devtools/client/debugger/src/selectors/breakpoints.js $(srcdir)/devtools/client/debugger/src/selectors/breakpointSources.js $(srcdir)/devtools/client/debugger/src/selectors/getCallStackFrames.js $(srcdir)/devtools/client/debugger/src/selectors/inComponent.js $(srcdir)/devtools/client/debugger/src/selectors/index.js $(srcdir)/devtools/client/debugger/src/selectors/isLineInScope.js $(srcdir)/devtools/client/debugger/src/selectors/isSelectedFrameVisible.js $(srcdir)/devtools/client/debugger/src/selectors/pause.js $(srcdir)/devtools/client/debugger/src/selectors/visibleBreakpoints.js $(srcdir)/devtools/client/debugger/src/selectors/visibleColumnBreakpoints.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/selectors)
	@$(TOUCH) $@

devtools/client/debugger/src/utils/misc: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/utils/node.stub: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/assert.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/ast.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/async-value.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/bootstrap.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/build-query.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/clipboard.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/connect.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/context.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/dbg.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/defer.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/DevToolsUtils.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/environment.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/expressions.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/evaluation-result.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/function.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/indentation.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/isMinified.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/location.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/log.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/memoize.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/memoizeLast.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/memoizableAction.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/path.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/prefs.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/preview.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/project-search.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/quick-open.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/result-list.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/selected-location.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/source-maps.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/source-queue.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/source.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/tabs.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/task.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/telemetry.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/text.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/timings.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/ui.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/url.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/utils.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/wasm.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/worker.js: devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/utils/assert.js $(srcdir)/devtools/client/debugger/src/utils/ast.js $(srcdir)/devtools/client/debugger/src/utils/async-value.js $(srcdir)/devtools/client/debugger/src/utils/bootstrap.js $(srcdir)/devtools/client/debugger/src/utils/build-query.js $(srcdir)/devtools/client/debugger/src/utils/clipboard.js $(srcdir)/devtools/client/debugger/src/utils/connect.js $(srcdir)/devtools/client/debugger/src/utils/context.js $(srcdir)/devtools/client/debugger/src/utils/dbg.js $(srcdir)/devtools/client/debugger/src/utils/defer.js $(srcdir)/devtools/client/debugger/src/utils/DevToolsUtils.js $(srcdir)/devtools/client/debugger/src/utils/environment.js $(srcdir)/devtools/client/debugger/src/utils/expressions.js $(srcdir)/devtools/client/debugger/src/utils/evaluation-result.js $(srcdir)/devtools/client/debugger/src/utils/function.js $(srcdir)/devtools/client/debugger/src/utils/indentation.js $(srcdir)/devtools/client/debugger/src/utils/isMinified.js $(srcdir)/devtools/client/debugger/src/utils/location.js $(srcdir)/devtools/client/debugger/src/utils/log.js $(srcdir)/devtools/client/debugger/src/utils/memoize.js $(srcdir)/devtools/client/debugger/src/utils/memoizeLast.js $(srcdir)/devtools/client/debugger/src/utils/memoizableAction.js $(srcdir)/devtools/client/debugger/src/utils/path.js $(srcdir)/devtools/client/debugger/src/utils/prefs.js $(srcdir)/devtools/client/debugger/src/utils/preview.js $(srcdir)/devtools/client/debugger/src/utils/project-search.js $(srcdir)/devtools/client/debugger/src/utils/quick-open.js $(srcdir)/devtools/client/debugger/src/utils/result-list.js $(srcdir)/devtools/client/debugger/src/utils/selected-location.js $(srcdir)/devtools/client/debugger/src/utils/source-maps.js $(srcdir)/devtools/client/debugger/src/utils/source-queue.js $(srcdir)/devtools/client/debugger/src/utils/source.js $(srcdir)/devtools/client/debugger/src/utils/tabs.js $(srcdir)/devtools/client/debugger/src/utils/task.js $(srcdir)/devtools/client/debugger/src/utils/telemetry.js $(srcdir)/devtools/client/debugger/src/utils/text.js $(srcdir)/devtools/client/debugger/src/utils/timings.js $(srcdir)/devtools/client/debugger/src/utils/ui.js $(srcdir)/devtools/client/debugger/src/utils/url.js $(srcdir)/devtools/client/debugger/src/utils/utils.js $(srcdir)/devtools/client/debugger/src/utils/wasm.js $(srcdir)/devtools/client/debugger/src/utils/worker.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/utils/node.stub devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/utils/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/utils/assert.js $(srcdir)/devtools/client/debugger/src/utils/ast.js $(srcdir)/devtools/client/debugger/src/utils/async-value.js $(srcdir)/devtools/client/debugger/src/utils/bootstrap.js $(srcdir)/devtools/client/debugger/src/utils/build-query.js $(srcdir)/devtools/client/debugger/src/utils/clipboard.js $(srcdir)/devtools/client/debugger/src/utils/connect.js $(srcdir)/devtools/client/debugger/src/utils/context.js $(srcdir)/devtools/client/debugger/src/utils/dbg.js $(srcdir)/devtools/client/debugger/src/utils/defer.js $(srcdir)/devtools/client/debugger/src/utils/DevToolsUtils.js $(srcdir)/devtools/client/debugger/src/utils/environment.js $(srcdir)/devtools/client/debugger/src/utils/expressions.js $(srcdir)/devtools/client/debugger/src/utils/evaluation-result.js $(srcdir)/devtools/client/debugger/src/utils/function.js $(srcdir)/devtools/client/debugger/src/utils/indentation.js $(srcdir)/devtools/client/debugger/src/utils/isMinified.js $(srcdir)/devtools/client/debugger/src/utils/location.js $(srcdir)/devtools/client/debugger/src/utils/log.js $(srcdir)/devtools/client/debugger/src/utils/memoize.js $(srcdir)/devtools/client/debugger/src/utils/memoizeLast.js $(srcdir)/devtools/client/debugger/src/utils/memoizableAction.js $(srcdir)/devtools/client/debugger/src/utils/path.js $(srcdir)/devtools/client/debugger/src/utils/prefs.js $(srcdir)/devtools/client/debugger/src/utils/preview.js $(srcdir)/devtools/client/debugger/src/utils/project-search.js $(srcdir)/devtools/client/debugger/src/utils/quick-open.js $(srcdir)/devtools/client/debugger/src/utils/result-list.js $(srcdir)/devtools/client/debugger/src/utils/selected-location.js $(srcdir)/devtools/client/debugger/src/utils/source-maps.js $(srcdir)/devtools/client/debugger/src/utils/source-queue.js $(srcdir)/devtools/client/debugger/src/utils/source.js $(srcdir)/devtools/client/debugger/src/utils/tabs.js $(srcdir)/devtools/client/debugger/src/utils/task.js $(srcdir)/devtools/client/debugger/src/utils/telemetry.js $(srcdir)/devtools/client/debugger/src/utils/text.js $(srcdir)/devtools/client/debugger/src/utils/timings.js $(srcdir)/devtools/client/debugger/src/utils/ui.js $(srcdir)/devtools/client/debugger/src/utils/url.js $(srcdir)/devtools/client/debugger/src/utils/utils.js $(srcdir)/devtools/client/debugger/src/utils/wasm.js $(srcdir)/devtools/client/debugger/src/utils/worker.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils)
	@$(TOUCH) $@

devtools/client/debugger/src/utils/breakpoint/misc: devtools/client/debugger/src/utils/breakpoint/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/utils/breakpoint/node.stub: devtools/client/debugger/src/utils/breakpoint/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/breakpoint/astBreakpointLocation.js: devtools/client/debugger/src/utils/breakpoint/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/breakpoint/breakpointPositions.js: devtools/client/debugger/src/utils/breakpoint/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/breakpoint/index.js: devtools/client/debugger/src/utils/breakpoint/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/utils/breakpoint/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/utils/breakpoint/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/utils/breakpoint/astBreakpointLocation.js $(srcdir)/devtools/client/debugger/src/utils/breakpoint/breakpointPositions.js $(srcdir)/devtools/client/debugger/src/utils/breakpoint/index.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/utils/breakpoint/node.stub devtools/client/debugger/src/utils/breakpoint/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/utils/breakpoint/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/utils/breakpoint/astBreakpointLocation.js $(srcdir)/devtools/client/debugger/src/utils/breakpoint/breakpointPositions.js $(srcdir)/devtools/client/debugger/src/utils/breakpoint/index.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/breakpoint)
	@$(TOUCH) $@

devtools/client/debugger/src/utils/editor/misc: devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/utils/editor/node.stub: devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/editor/create-editor.js: devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/editor/get-expression.js: devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/editor/get-token-location.js: devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/editor/index.js: devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/editor/source-documents.js: devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/editor/source-editor.js: devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/editor/source-search.js: devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/editor/token-events.js: devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/utils/editor/create-editor.js $(srcdir)/devtools/client/debugger/src/utils/editor/get-expression.js $(srcdir)/devtools/client/debugger/src/utils/editor/get-token-location.js $(srcdir)/devtools/client/debugger/src/utils/editor/index.js $(srcdir)/devtools/client/debugger/src/utils/editor/source-documents.js $(srcdir)/devtools/client/debugger/src/utils/editor/source-editor.js $(srcdir)/devtools/client/debugger/src/utils/editor/source-search.js $(srcdir)/devtools/client/debugger/src/utils/editor/token-events.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/utils/editor/node.stub devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/utils/editor/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/utils/editor/create-editor.js $(srcdir)/devtools/client/debugger/src/utils/editor/get-expression.js $(srcdir)/devtools/client/debugger/src/utils/editor/get-token-location.js $(srcdir)/devtools/client/debugger/src/utils/editor/index.js $(srcdir)/devtools/client/debugger/src/utils/editor/source-documents.js $(srcdir)/devtools/client/debugger/src/utils/editor/source-editor.js $(srcdir)/devtools/client/debugger/src/utils/editor/source-search.js $(srcdir)/devtools/client/debugger/src/utils/editor/token-events.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/editor)
	@$(TOUCH) $@

devtools/client/debugger/src/utils/pause/misc: devtools/client/debugger/src/utils/pause/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/utils/pause/node.stub: devtools/client/debugger/src/utils/pause/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/index.js: devtools/client/debugger/src/utils/pause/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/why.js: devtools/client/debugger/src/utils/pause/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/utils/pause/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/utils/pause/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/utils/pause/index.js $(srcdir)/devtools/client/debugger/src/utils/pause/why.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/utils/pause/node.stub devtools/client/debugger/src/utils/pause/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/utils/pause/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/utils/pause/index.js $(srcdir)/devtools/client/debugger/src/utils/pause/why.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause)
	@$(TOUCH) $@

devtools/client/debugger/src/utils/pause/frames/misc: devtools/client/debugger/src/utils/pause/frames/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/utils/pause/frames/node.stub: devtools/client/debugger/src/utils/pause/frames/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/frames/annotateFrames.js: devtools/client/debugger/src/utils/pause/frames/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/frames/collapseFrames.js: devtools/client/debugger/src/utils/pause/frames/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/frames/displayName.js: devtools/client/debugger/src/utils/pause/frames/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/frames/getFrameUrl.js: devtools/client/debugger/src/utils/pause/frames/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/frames/getLibraryFromUrl.js: devtools/client/debugger/src/utils/pause/frames/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/frames/index.js: devtools/client/debugger/src/utils/pause/frames/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/utils/pause/frames/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/utils/pause/frames/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/utils/pause/frames/annotateFrames.js $(srcdir)/devtools/client/debugger/src/utils/pause/frames/collapseFrames.js $(srcdir)/devtools/client/debugger/src/utils/pause/frames/displayName.js $(srcdir)/devtools/client/debugger/src/utils/pause/frames/getFrameUrl.js $(srcdir)/devtools/client/debugger/src/utils/pause/frames/getLibraryFromUrl.js $(srcdir)/devtools/client/debugger/src/utils/pause/frames/index.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/utils/pause/frames/node.stub devtools/client/debugger/src/utils/pause/frames/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/utils/pause/frames/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/utils/pause/frames/annotateFrames.js $(srcdir)/devtools/client/debugger/src/utils/pause/frames/collapseFrames.js $(srcdir)/devtools/client/debugger/src/utils/pause/frames/displayName.js $(srcdir)/devtools/client/debugger/src/utils/pause/frames/getFrameUrl.js $(srcdir)/devtools/client/debugger/src/utils/pause/frames/getLibraryFromUrl.js $(srcdir)/devtools/client/debugger/src/utils/pause/frames/index.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/frames)
	@$(TOUCH) $@

devtools/client/debugger/src/utils/pause/mapScopes/misc: devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/utils/pause/mapScopes/node.stub: devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/mapScopes/buildGeneratedBindingList.js: devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/mapScopes/filtering.js: devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/mapScopes/findGeneratedBindingFromPosition.js: devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/mapScopes/getApplicableBindingsForOriginalPosition.js: devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/mapScopes/index.js: devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/mapScopes/locColumn.js: devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/mapScopes/mappingContains.js: devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/mapScopes/optimizedOut.js: devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/mapScopes/positionCmp.js: devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/mapScopes/rangeMetadata.js: devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/utils/pause/mapScopes/buildGeneratedBindingList.js $(srcdir)/devtools/client/debugger/src/utils/pause/mapScopes/filtering.js $(srcdir)/devtools/client/debugger/src/utils/pause/mapScopes/findGeneratedBindingFromPosition.js $(srcdir)/devtools/client/debugger/src/utils/pause/mapScopes/getApplicableBindingsForOriginalPosition.js $(srcdir)/devtools/client/debugger/src/utils/pause/mapScopes/index.js $(srcdir)/devtools/client/debugger/src/utils/pause/mapScopes/locColumn.js $(srcdir)/devtools/client/debugger/src/utils/pause/mapScopes/mappingContains.js $(srcdir)/devtools/client/debugger/src/utils/pause/mapScopes/optimizedOut.js $(srcdir)/devtools/client/debugger/src/utils/pause/mapScopes/positionCmp.js $(srcdir)/devtools/client/debugger/src/utils/pause/mapScopes/rangeMetadata.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/utils/pause/mapScopes/node.stub devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/utils/pause/mapScopes/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/utils/pause/mapScopes/buildGeneratedBindingList.js $(srcdir)/devtools/client/debugger/src/utils/pause/mapScopes/filtering.js $(srcdir)/devtools/client/debugger/src/utils/pause/mapScopes/findGeneratedBindingFromPosition.js $(srcdir)/devtools/client/debugger/src/utils/pause/mapScopes/getApplicableBindingsForOriginalPosition.js $(srcdir)/devtools/client/debugger/src/utils/pause/mapScopes/index.js $(srcdir)/devtools/client/debugger/src/utils/pause/mapScopes/locColumn.js $(srcdir)/devtools/client/debugger/src/utils/pause/mapScopes/mappingContains.js $(srcdir)/devtools/client/debugger/src/utils/pause/mapScopes/optimizedOut.js $(srcdir)/devtools/client/debugger/src/utils/pause/mapScopes/positionCmp.js $(srcdir)/devtools/client/debugger/src/utils/pause/mapScopes/rangeMetadata.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/mapScopes)
	@$(TOUCH) $@

devtools/client/debugger/src/utils/pause/scopes/misc: devtools/client/debugger/src/utils/pause/scopes/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/utils/pause/scopes/node.stub: devtools/client/debugger/src/utils/pause/scopes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/scopes/getScope.js: devtools/client/debugger/src/utils/pause/scopes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/scopes/getVariables.js: devtools/client/debugger/src/utils/pause/scopes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/scopes/index.js: devtools/client/debugger/src/utils/pause/scopes/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/scopes/utils.js: devtools/client/debugger/src/utils/pause/scopes/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/utils/pause/scopes/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/utils/pause/scopes/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/utils/pause/scopes/getScope.js $(srcdir)/devtools/client/debugger/src/utils/pause/scopes/getVariables.js $(srcdir)/devtools/client/debugger/src/utils/pause/scopes/index.js $(srcdir)/devtools/client/debugger/src/utils/pause/scopes/utils.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/utils/pause/scopes/node.stub devtools/client/debugger/src/utils/pause/scopes/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/utils/pause/scopes/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/utils/pause/scopes/getScope.js $(srcdir)/devtools/client/debugger/src/utils/pause/scopes/getVariables.js $(srcdir)/devtools/client/debugger/src/utils/pause/scopes/index.js $(srcdir)/devtools/client/debugger/src/utils/pause/scopes/utils.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/pause/scopes)
	@$(TOUCH) $@

devtools/client/debugger/src/utils/resource/misc: devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/utils/resource/node.stub: devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/resource/base-query.js: devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/resource/compare.js: devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/resource/core.js: devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/resource/index.js: devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/resource/memoize.js: devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/resource/query-cache.js: devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/resource/query.js: devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/resource/selector.js: devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/utils/resource/base-query.js $(srcdir)/devtools/client/debugger/src/utils/resource/compare.js $(srcdir)/devtools/client/debugger/src/utils/resource/core.js $(srcdir)/devtools/client/debugger/src/utils/resource/index.js $(srcdir)/devtools/client/debugger/src/utils/resource/memoize.js $(srcdir)/devtools/client/debugger/src/utils/resource/query-cache.js $(srcdir)/devtools/client/debugger/src/utils/resource/query.js $(srcdir)/devtools/client/debugger/src/utils/resource/selector.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/utils/resource/node.stub devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/utils/resource/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/utils/resource/base-query.js $(srcdir)/devtools/client/debugger/src/utils/resource/compare.js $(srcdir)/devtools/client/debugger/src/utils/resource/core.js $(srcdir)/devtools/client/debugger/src/utils/resource/index.js $(srcdir)/devtools/client/debugger/src/utils/resource/memoize.js $(srcdir)/devtools/client/debugger/src/utils/resource/query-cache.js $(srcdir)/devtools/client/debugger/src/utils/resource/query.js $(srcdir)/devtools/client/debugger/src/utils/resource/selector.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/resource)
	@$(TOUCH) $@

devtools/client/debugger/src/utils/sources-tree/misc: devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/utils/sources-tree/node.stub: devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/sources-tree/addToTree.js: devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/sources-tree/collapseTree.js: devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/sources-tree/formatTree.js: devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/sources-tree/getDirectories.js: devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/sources-tree/getURL.js: devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/sources-tree/index.js: devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/sources-tree/sortTree.js: devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/sources-tree/treeOrder.js: devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/sources-tree/updateTree.js: devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/sources-tree/utils.js: devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/utils/sources-tree/addToTree.js $(srcdir)/devtools/client/debugger/src/utils/sources-tree/collapseTree.js $(srcdir)/devtools/client/debugger/src/utils/sources-tree/formatTree.js $(srcdir)/devtools/client/debugger/src/utils/sources-tree/getDirectories.js $(srcdir)/devtools/client/debugger/src/utils/sources-tree/getURL.js $(srcdir)/devtools/client/debugger/src/utils/sources-tree/index.js $(srcdir)/devtools/client/debugger/src/utils/sources-tree/sortTree.js $(srcdir)/devtools/client/debugger/src/utils/sources-tree/treeOrder.js $(srcdir)/devtools/client/debugger/src/utils/sources-tree/updateTree.js $(srcdir)/devtools/client/debugger/src/utils/sources-tree/utils.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/utils/sources-tree/node.stub devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/utils/sources-tree/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/utils/sources-tree/addToTree.js $(srcdir)/devtools/client/debugger/src/utils/sources-tree/collapseTree.js $(srcdir)/devtools/client/debugger/src/utils/sources-tree/formatTree.js $(srcdir)/devtools/client/debugger/src/utils/sources-tree/getDirectories.js $(srcdir)/devtools/client/debugger/src/utils/sources-tree/getURL.js $(srcdir)/devtools/client/debugger/src/utils/sources-tree/index.js $(srcdir)/devtools/client/debugger/src/utils/sources-tree/sortTree.js $(srcdir)/devtools/client/debugger/src/utils/sources-tree/treeOrder.js $(srcdir)/devtools/client/debugger/src/utils/sources-tree/updateTree.js $(srcdir)/devtools/client/debugger/src/utils/sources-tree/utils.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/utils/sources-tree)
	@$(TOUCH) $@

devtools/client/debugger/src/workers/misc: devtools/client/debugger/src/workers/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/workers/node.stub: devtools/client/debugger/src/workers/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/workers/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/workers/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/workers/node.stub devtools/client/debugger/src/workers/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/workers/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/workers)
	@$(TOUCH) $@

devtools/client/debugger/src/workers/parser/misc: devtools/client/debugger/src/workers/parser/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/workers/parser/node.stub: devtools/client/debugger/src/workers/parser/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/workers/parser/index.js: devtools/client/debugger/src/workers/parser/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/workers/parser/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/workers/parser/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/workers/parser/index.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/workers/parser/node.stub devtools/client/debugger/src/workers/parser/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/workers/parser/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/workers/parser/index.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/workers/parser)
	@$(TOUCH) $@

devtools/client/debugger/src/workers/pretty-print/misc: devtools/client/debugger/src/workers/pretty-print/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/workers/pretty-print/node.stub: devtools/client/debugger/src/workers/pretty-print/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/workers/pretty-print/index.js: devtools/client/debugger/src/workers/pretty-print/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/workers/pretty-print/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/workers/pretty-print/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/workers/pretty-print/index.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/workers/pretty-print/node.stub devtools/client/debugger/src/workers/pretty-print/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/workers/pretty-print/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/workers/pretty-print/index.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/workers/pretty-print)
	@$(TOUCH) $@

devtools/client/debugger/src/workers/search/misc: devtools/client/debugger/src/workers/search/$(MDDEPDIR)/node.stub.stub
devtools/client/debugger/src/workers/search/node.stub: devtools/client/debugger/src/workers/search/$(MDDEPDIR)/node.stub.stub ;
worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/workers/search/index.js: devtools/client/debugger/src/workers/search/$(MDDEPDIR)/node.stub.stub ;
EXTRA_MDDEPEND_FILES += devtools/client/debugger/src/workers/search/$(MDDEPDIR)/node.stub.pp
devtools/client/debugger/src/workers/search/$(MDDEPDIR)/node.stub.stub: /worker/build/python/mozbuild/mozbuild/action/node.py $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/workers/search/index.js backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/node.py generate devtools/client/debugger/src/workers/search/node.stub devtools/client/debugger/src/workers/search/$(MDDEPDIR)/node.stub.pp devtools/client/debugger/src/workers/search/$(MDDEPDIR)/node.stub.stub $(srcdir)/devtools/client/shared/build/build.js $(srcdir)/devtools/client/debugger/src/workers/search/index.js /worker/build/obj-x86_64-pc-linux-gnu/dist/bin/browser/chrome/devtools/modules/devtools/client/debugger/src/workers/search)
	@$(TOUCH) $@

devtools/shared/webconsole/misc: devtools/shared/webconsole/$(MDDEPDIR)/reserved-js-words.js.stub
devtools/shared/webconsole/reserved-js-words.js: devtools/shared/webconsole/$(MDDEPDIR)/reserved-js-words.js.stub ;
EXTRA_MDDEPEND_FILES += devtools/shared/webconsole/$(MDDEPDIR)/reserved-js-words.js.pp
devtools/shared/webconsole/$(MDDEPDIR)/reserved-js-words.js.stub: /worker/build/devtools/shared/webconsole/GenerateReservedWordsJS.py $(srcdir)/js/src/frontend/ReservedWords.h
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/devtools/shared/webconsole/GenerateReservedWordsJS.py main devtools/shared/webconsole/reserved-js-words.js devtools/shared/webconsole/$(MDDEPDIR)/reserved-js-words.js.pp devtools/shared/webconsole/$(MDDEPDIR)/reserved-js-words.js.stub $(srcdir)/js/src/frontend/ReservedWords.h)
	@$(TOUCH) $@

toolkit/library/build/misc: toolkit/library/build/$(MDDEPDIR)/dependentlibs.list.stub
toolkit/library/build/dependentlibs.list: toolkit/library/build/$(MDDEPDIR)/dependentlibs.list.stub ;
toolkit/library/build/dependentlibs.list.gtest: toolkit/library/build/$(MDDEPDIR)/dependentlibs.list.stub ;
EXTRA_MDDEPEND_FILES += toolkit/library/build/$(MDDEPDIR)/dependentlibs.list.pp
toolkit/library/build/$(MDDEPDIR)/dependentlibs.list.stub: /worker/build/toolkit/library/build/dependentlibs.py toolkit/library/build/libxul.so
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/library/build/dependentlibs.py gen_list toolkit/library/build/dependentlibs.list toolkit/library/build/$(MDDEPDIR)/dependentlibs.list.pp toolkit/library/build/$(MDDEPDIR)/dependentlibs.list.stub toolkit/library/build/libxul.so)
	@$(TOUCH) $@

services/settings/test/unit/test_remote_settings_signatures/misc: services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_root.pem.stub
services/settings/test/unit/test_remote_settings_signatures/collection_signing_root.pem: services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_root.pem.stub ;
EXTRA_MDDEPEND_FILES += services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_root.pem.pp
services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_root.pem.stub: /worker/build/security/manager/ssl/tests/unit/pycert.py $(srcdir)/services/settings/test/unit/test_remote_settings_signatures/collection_signing_root.pem.certspec
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/manager/ssl/tests/unit/pycert.py main services/settings/test/unit/test_remote_settings_signatures/collection_signing_root.pem services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_root.pem.pp services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_root.pem.stub $(srcdir)/services/settings/test/unit/test_remote_settings_signatures/collection_signing_root.pem.certspec)
	@$(TOUCH) $@

services/settings/test/unit/test_remote_settings_signatures/misc: services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_int.pem.stub
services/settings/test/unit/test_remote_settings_signatures/collection_signing_int.pem: services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_int.pem.stub ;
EXTRA_MDDEPEND_FILES += services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_int.pem.pp
services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_int.pem.stub: /worker/build/security/manager/ssl/tests/unit/pycert.py $(srcdir)/services/settings/test/unit/test_remote_settings_signatures/collection_signing_int.pem.certspec
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/manager/ssl/tests/unit/pycert.py main services/settings/test/unit/test_remote_settings_signatures/collection_signing_int.pem services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_int.pem.pp services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_int.pem.stub $(srcdir)/services/settings/test/unit/test_remote_settings_signatures/collection_signing_int.pem.certspec)
	@$(TOUCH) $@

services/settings/test/unit/test_remote_settings_signatures/misc: services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_ee.pem.stub
services/settings/test/unit/test_remote_settings_signatures/collection_signing_ee.pem: services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_ee.pem.stub ;
EXTRA_MDDEPEND_FILES += services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_ee.pem.pp
services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_ee.pem.stub: /worker/build/security/manager/ssl/tests/unit/pycert.py $(srcdir)/services/settings/test/unit/test_remote_settings_signatures/collection_signing_ee.pem.certspec
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/manager/ssl/tests/unit/pycert.py main services/settings/test/unit/test_remote_settings_signatures/collection_signing_ee.pem services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_ee.pem.pp services/settings/test/unit/test_remote_settings_signatures/$(MDDEPDIR)/collection_signing_ee.pem.stub $(srcdir)/services/settings/test/unit/test_remote_settings_signatures/collection_signing_ee.pem.certspec)
	@$(TOUCH) $@

browser/base/misc: browser/base/content/$(MDDEPDIR)/aboutNetErrorCodes.js.stub
browser/base/content/aboutNetErrorCodes.js: browser/base/content/$(MDDEPDIR)/aboutNetErrorCodes.js.stub ;
EXTRA_MDDEPEND_FILES += browser/base/content/$(MDDEPDIR)/aboutNetErrorCodes.js.pp
browser/base/content/$(MDDEPDIR)/aboutNetErrorCodes.js.stub: /worker/build/browser/base/gen_aboutneterror_codes.py $(srcdir)/browser/locales/en-US/browser/nsserrors.ftl
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/browser/base/gen_aboutneterror_codes.py main browser/base/content/aboutNetErrorCodes.js browser/base/content/$(MDDEPDIR)/aboutNetErrorCodes.js.pp browser/base/content/$(MDDEPDIR)/aboutNetErrorCodes.js.stub $(srcdir)/browser/locales/en-US/browser/nsserrors.ftl)
	@$(TOUCH) $@

browser/app/macbuild/Contents/misc: browser/app/macbuild/Contents/$(MDDEPDIR)/MacOS-files.txt.stub
browser/app/macbuild/Contents/MacOS-files.txt: browser/app/macbuild/Contents/$(MDDEPDIR)/MacOS-files.txt.stub ;
EXTRA_MDDEPEND_FILES += browser/app/macbuild/Contents/$(MDDEPDIR)/MacOS-files.txt.pp
browser/app/macbuild/Contents/$(MDDEPDIR)/MacOS-files.txt.stub: /worker/build/python/mozbuild/mozbuild/action/preprocessor.py $(srcdir)/browser/app/macbuild/Contents/MacOS-files.in backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/preprocessor.py generate browser/app/macbuild/Contents/MacOS-files.txt browser/app/macbuild/Contents/$(MDDEPDIR)/MacOS-files.txt.pp browser/app/macbuild/Contents/$(MDDEPDIR)/MacOS-files.txt.stub $(srcdir)/browser/app/macbuild/Contents/MacOS-files.in -DMOZ_CRASHREPORTER=1 -DMOZ_APP_NAME=konke)
	@$(TOUCH) $@

