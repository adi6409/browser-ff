nonstatic_webidl_files := Animation.webidl CSS2Properties.webidl ChromeUtils.webidl Node.webidl TestCodeGen.webidl TestExampleGen.webidl TestJSImplGen.webidl Window.webidl
globalgen_sources := PrototypeList.cpp RegisterBindings.cpp RegisterWorkerBindings.cpp RegisterWorkerDebuggerBindings.cpp RegisterWorkletBindings.cpp UnionTypes.cpp WebIDLPrefs.cpp WebIDLSerializable.cpp
test_sources := TestCodeGenBinding.cpp TestDictionaryBinding.cpp TestExampleGenBinding.cpp TestJSImplGenBinding.cpp TestJSImplInheritanceGenBinding.cpp TestTypedefBinding.cpp
TestCodeGen.webidl: /worker/build/dom/bindings/test/TestCodeGen.webidl $(WEBIDL_PP_DEPS)
	$(RM) $@
	$(call py_action,preprocessor,$(DEFINES) $(ACDEFINES) $< -o $@)
TestExampleGen.webidl: /worker/build/dom/bindings/test/TestExampleGen.webidl $(WEBIDL_PP_DEPS)
	$(RM) $@
	$(call py_action,preprocessor,$(DEFINES) $(ACDEFINES) $< -o $@)
TestJSImplGen.webidl: /worker/build/dom/bindings/test/TestJSImplGen.webidl $(WEBIDL_PP_DEPS)
	$(RM) $@
	$(call py_action,preprocessor,$(DEFINES) $(ACDEFINES) $< -o $@)
ChromeUtils.webidl: /worker/build/dom/chrome-webidl/ChromeUtils.webidl $(WEBIDL_PP_DEPS)
	$(RM) $@
	$(call py_action,preprocessor,$(DEFINES) $(ACDEFINES) $< -o $@)
Animation.webidl: /worker/build/dom/webidl/Animation.webidl $(WEBIDL_PP_DEPS)
	$(RM) $@
	$(call py_action,preprocessor,$(DEFINES) $(ACDEFINES) $< -o $@)
Node.webidl: /worker/build/dom/webidl/Node.webidl $(WEBIDL_PP_DEPS)
	$(RM) $@
	$(call py_action,preprocessor,$(DEFINES) $(ACDEFINES) $< -o $@)
Window.webidl: /worker/build/dom/webidl/Window.webidl $(WEBIDL_PP_DEPS)
	$(RM) $@
	$(call py_action,preprocessor,$(DEFINES) $(ACDEFINES) $< -o $@)

# We build files in 'unified' mode by including several files
# together into a single source file.  This cuts down on
# compilation times and debug information size.
unified_binding_cpp_files := UnifiedBindings0.cpp UnifiedBindings1.cpp UnifiedBindings10.cpp UnifiedBindings11.cpp UnifiedBindings12.cpp UnifiedBindings13.cpp UnifiedBindings14.cpp UnifiedBindings15.cpp UnifiedBindings16.cpp UnifiedBindings17.cpp UnifiedBindings18.cpp UnifiedBindings19.cpp UnifiedBindings2.cpp UnifiedBindings20.cpp UnifiedBindings21.cpp UnifiedBindings22.cpp UnifiedBindings23.cpp UnifiedBindings24.cpp UnifiedBindings25.cpp UnifiedBindings3.cpp UnifiedBindings4.cpp UnifiedBindings5.cpp UnifiedBindings6.cpp UnifiedBindings7.cpp UnifiedBindings8.cpp UnifiedBindings9.cpp

# Make sometimes gets confused between "foo" and "$(CURDIR)/foo".
# Help it out by explicitly specifiying dependencies.
all_absolute_unified_files := \
  $(addprefix $(CURDIR)/,$(unified_binding_cpp_files))
$(all_absolute_unified_files): $(CURDIR)/%: %
