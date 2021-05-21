/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/sandbox/common/mozISandboxSettings.idl
 */

#ifndef __gen_mozISandboxSettings_h__
#define __gen_mozISandboxSettings_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    mozISandboxSettings */
#define MOZISANDBOXSETTINGS_IID_STR "5516303d-9007-45a0-94b9-940ef134a6e2"

#define MOZISANDBOXSETTINGS_IID \
  {0x5516303d, 0x9007, 0x45a0, \
    { 0x94, 0xb9, 0x94, 0x0e, 0xf1, 0x34, 0xa6, 0xe2 }}

class NS_NO_VTABLE mozISandboxSettings : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZISANDBOXSETTINGS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozISandboxSettings;

  /* readonly attribute long effectiveContentSandboxLevel; */
  NS_IMETHOD GetEffectiveContentSandboxLevel(int32_t *aEffectiveContentSandboxLevel) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozISandboxSettings, MOZISANDBOXSETTINGS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZISANDBOXSETTINGS \
  NS_IMETHOD GetEffectiveContentSandboxLevel(int32_t *aEffectiveContentSandboxLevel) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZISANDBOXSETTINGS \
  nsresult GetEffectiveContentSandboxLevel(int32_t *aEffectiveContentSandboxLevel); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZISANDBOXSETTINGS(_to) \
  NS_IMETHOD GetEffectiveContentSandboxLevel(int32_t *aEffectiveContentSandboxLevel) override { return _to GetEffectiveContentSandboxLevel(aEffectiveContentSandboxLevel); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZISANDBOXSETTINGS(_to) \
  NS_IMETHOD GetEffectiveContentSandboxLevel(int32_t *aEffectiveContentSandboxLevel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEffectiveContentSandboxLevel(aEffectiveContentSandboxLevel); } 


#define MOZ_SANDBOX_SETTINGS_CID \
{0x5516303d, 0x9007, 0x45a0, { 0x94, 0xb9, 0x94, 0x0e, 0xf1, 0x34, 0xa6, 0xe2}}
#define MOZ_SANDBOX_SETTINGS_CONTRACTID \
    "@mozilla.org/sandbox/sandbox-settings;1"

#endif /* __gen_mozISandboxSettings_h__ */
