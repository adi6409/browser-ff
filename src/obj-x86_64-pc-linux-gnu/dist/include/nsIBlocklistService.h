/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsIBlocklistService.idl
 */

#ifndef __gen_nsIBlocklistService_h__
#define __gen_nsIBlocklistService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPluginTag; /* forward declaration */

class nsIVariant; /* forward declaration */


/* starting interface:    nsIBlocklistService */
#define NS_IBLOCKLISTSERVICE_IID_STR "a6dcc76e-9f62-4cc1-a470-b483a1a6f096"

#define NS_IBLOCKLISTSERVICE_IID \
  {0xa6dcc76e, 0x9f62, 0x4cc1, \
    { 0xa4, 0x70, 0xb4, 0x83, 0xa1, 0xa6, 0xf0, 0x96 }}

class NS_NO_VTABLE nsIBlocklistService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IBLOCKLISTSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIBlocklistService;

  enum {
    STATE_NOT_BLOCKED = 0U,
    STATE_SOFTBLOCKED = 1U,
    STATE_BLOCKED = 2U,
    STATE_OUTDATED = 3U,
    STATE_VULNERABLE_UPDATE_AVAILABLE = 4U,
    STATE_VULNERABLE_NO_UPDATE = 5U,
    STATE_MAX = 6U
  };

  /* Promise getPluginBlocklistState (in nsIPluginTag plugin, [optional] in AString appVersion, [optional] in AString toolkitVersion); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPluginBlocklistState(nsIPluginTag *plugin, const nsAString& appVersion, const nsAString& toolkitVersion, ::mozilla::dom::Promise * * _retval) = 0;

  /* readonly attribute boolean isLoaded; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsLoaded(bool *aIsLoaded) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIBlocklistService, NS_IBLOCKLISTSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIBLOCKLISTSERVICE \
  NS_IMETHOD GetPluginBlocklistState(nsIPluginTag *plugin, const nsAString& appVersion, const nsAString& toolkitVersion, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetIsLoaded(bool *aIsLoaded) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIBLOCKLISTSERVICE \
  nsresult GetPluginBlocklistState(nsIPluginTag *plugin, const nsAString& appVersion, const nsAString& toolkitVersion, ::mozilla::dom::Promise * * _retval); \
  nsresult GetIsLoaded(bool *aIsLoaded); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIBLOCKLISTSERVICE(_to) \
  NS_IMETHOD GetPluginBlocklistState(nsIPluginTag *plugin, const nsAString& appVersion, const nsAString& toolkitVersion, ::mozilla::dom::Promise * * _retval) override { return _to GetPluginBlocklistState(plugin, appVersion, toolkitVersion, _retval); } \
  NS_IMETHOD GetIsLoaded(bool *aIsLoaded) override { return _to GetIsLoaded(aIsLoaded); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIBLOCKLISTSERVICE(_to) \
  NS_IMETHOD GetPluginBlocklistState(nsIPluginTag *plugin, const nsAString& appVersion, const nsAString& toolkitVersion, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPluginBlocklistState(plugin, appVersion, toolkitVersion, _retval); } \
  NS_IMETHOD GetIsLoaded(bool *aIsLoaded) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsLoaded(aIsLoaded); } 


#endif /* __gen_nsIBlocklistService_h__ */
