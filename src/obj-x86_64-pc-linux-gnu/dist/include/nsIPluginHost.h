/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/plugins/base/nsIPluginHost.idl
 */

#ifndef __gen_nsIPluginHost_h__
#define __gen_nsIPluginHost_h__


#ifndef __gen_nspluginroot_h__
#include "nspluginroot.h"
#endif

#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIPluginTag_h__
#include "nsIPluginTag.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#define MOZ_PLUGIN_HOST_CONTRACTID \
  "@mozilla.org/plugin/host;1"

/* starting interface:    nsIClearSiteDataCallback */
#define NS_ICLEARSITEDATACALLBACK_IID_STR "9c311778-7c2c-4ad8-b439-b8a2786a20dd"

#define NS_ICLEARSITEDATACALLBACK_IID \
  {0x9c311778, 0x7c2c, 0x4ad8, \
    { 0xb4, 0x39, 0xb8, 0xa2, 0x78, 0x6a, 0x20, 0xdd }}

class NS_NO_VTABLE nsIClearSiteDataCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICLEARSITEDATACALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIClearSiteDataCallback;

  /* void callback (in nsresult rv); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Callback(nsresult rv) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIClearSiteDataCallback, NS_ICLEARSITEDATACALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICLEARSITEDATACALLBACK \
  NS_IMETHOD Callback(nsresult rv) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICLEARSITEDATACALLBACK \
  nsresult Callback(nsresult rv); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICLEARSITEDATACALLBACK(_to) \
  NS_IMETHOD Callback(nsresult rv) override { return _to Callback(rv); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICLEARSITEDATACALLBACK(_to) \
  NS_IMETHOD Callback(nsresult rv) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Callback(rv); } 


/* starting interface:    nsIPluginHost */
#define NS_IPLUGINHOST_IID_STR "f938f5ba-7093-42cd-a559-af8039d99204"

#define NS_IPLUGINHOST_IID \
  {0xf938f5ba, 0x7093, 0x42cd, \
    { 0xa5, 0x59, 0xaf, 0x80, 0x39, 0xd9, 0x92, 0x04 }}

class NS_NO_VTABLE nsIPluginHost : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPLUGINHOST_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPluginHost;

  /* void reloadPlugins (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ReloadPlugins(void) = 0;

  /* Array<nsIPluginTag> getPluginTags (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPluginTags(nsTArray<RefPtr<nsIPluginTag>>& _retval) = 0;

  enum {
    FLAG_CLEAR_ALL = 0U,
    FLAG_CLEAR_CACHE = 1U,
    EXCLUDE_NONE = 0U,
    EXCLUDE_DISABLED = 1U,
    EXCLUDE_FAKE = 2U
  };

  /* void clearSiteData (in nsIPluginTag plugin, in AUTF8String domain, in uint64_t flags, in int64_t maxAge, in nsIClearSiteDataCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ClearSiteData(nsIPluginTag *plugin, const nsACString& domain, uint64_t flags, int64_t maxAge, nsIClearSiteDataCallback *callback) = 0;

  /* boolean siteHasData (in nsIPluginTag plugin, in AUTF8String domain); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SiteHasData(nsIPluginTag *plugin, const nsACString& domain, bool *_retval) = 0;

  /* ACString getPermissionStringForType (in AUTF8String mimeType, [optional] in uint32_t excludeFlags); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPermissionStringForType(const nsACString& mimeType, uint32_t excludeFlags, nsACString& _retval) = 0;

  /* ACString getPermissionStringForTag (in nsIPluginTag tag, [optional] in uint32_t excludeFlags); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPermissionStringForTag(nsIPluginTag *tag, uint32_t excludeFlags, nsACString& _retval) = 0;

  /* nsIPluginTag getPluginTagForType (in AUTF8String mimeType, [optional] in uint32_t excludeFlags); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPluginTagForType(const nsACString& mimeType, uint32_t excludeFlags, nsIPluginTag **_retval) = 0;

  /* unsigned long getStateForType (in AUTF8String mimeType, [optional] in uint32_t excludeFlags); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetStateForType(const nsACString& mimeType, uint32_t excludeFlags, uint32_t *_retval) = 0;

  /* uint32_t getBlocklistStateForType (in AUTF8String aMimeType, [optional] in uint32_t excludeFlags); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBlocklistStateForType(const nsACString& aMimeType, uint32_t excludeFlags, uint32_t *_retval) = 0;

  /* [implicit_jscontext] nsIFakePluginTag registerFakePlugin (in jsval initDictionary); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RegisterFakePlugin(JS::HandleValue initDictionary, JSContext* cx, nsIFakePluginTag **_retval) = 0;

  /* [implicit_jscontext] nsIFakePluginTag createFakePlugin (in jsval initDictionary); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CreateFakePlugin(JS::HandleValue initDictionary, JSContext* cx, nsIFakePluginTag **_retval) = 0;

  /* nsIFakePluginTag getFakePlugin (in AUTF8String mimeType); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFakePlugin(const nsACString& mimeType, nsIFakePluginTag **_retval) = 0;

  /* void unregisterFakePlugin (in AUTF8String handlerURI); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UnregisterFakePlugin(const nsACString& handlerURI) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPluginHost, NS_IPLUGINHOST_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPLUGINHOST \
  NS_IMETHOD ReloadPlugins(void) override; \
  NS_IMETHOD GetPluginTags(nsTArray<RefPtr<nsIPluginTag>>& _retval) override; \
  NS_IMETHOD ClearSiteData(nsIPluginTag *plugin, const nsACString& domain, uint64_t flags, int64_t maxAge, nsIClearSiteDataCallback *callback) override; \
  NS_IMETHOD SiteHasData(nsIPluginTag *plugin, const nsACString& domain, bool *_retval) override; \
  NS_IMETHOD GetPermissionStringForType(const nsACString& mimeType, uint32_t excludeFlags, nsACString& _retval) override; \
  NS_IMETHOD GetPermissionStringForTag(nsIPluginTag *tag, uint32_t excludeFlags, nsACString& _retval) override; \
  NS_IMETHOD GetPluginTagForType(const nsACString& mimeType, uint32_t excludeFlags, nsIPluginTag **_retval) override; \
  NS_IMETHOD GetStateForType(const nsACString& mimeType, uint32_t excludeFlags, uint32_t *_retval) override; \
  NS_IMETHOD GetBlocklistStateForType(const nsACString& aMimeType, uint32_t excludeFlags, uint32_t *_retval) override; \
  NS_IMETHOD RegisterFakePlugin(JS::HandleValue initDictionary, JSContext* cx, nsIFakePluginTag **_retval) override; \
  NS_IMETHOD CreateFakePlugin(JS::HandleValue initDictionary, JSContext* cx, nsIFakePluginTag **_retval) override; \
  NS_IMETHOD GetFakePlugin(const nsACString& mimeType, nsIFakePluginTag **_retval) override; \
  NS_IMETHOD UnregisterFakePlugin(const nsACString& handlerURI) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPLUGINHOST \
  nsresult ReloadPlugins(void); \
  nsresult GetPluginTags(nsTArray<RefPtr<nsIPluginTag>>& _retval); \
  nsresult ClearSiteData(nsIPluginTag *plugin, const nsACString& domain, uint64_t flags, int64_t maxAge, nsIClearSiteDataCallback *callback); \
  nsresult SiteHasData(nsIPluginTag *plugin, const nsACString& domain, bool *_retval); \
  nsresult GetPermissionStringForType(const nsACString& mimeType, uint32_t excludeFlags, nsACString& _retval); \
  nsresult GetPermissionStringForTag(nsIPluginTag *tag, uint32_t excludeFlags, nsACString& _retval); \
  nsresult GetPluginTagForType(const nsACString& mimeType, uint32_t excludeFlags, nsIPluginTag **_retval); \
  nsresult GetStateForType(const nsACString& mimeType, uint32_t excludeFlags, uint32_t *_retval); \
  nsresult GetBlocklistStateForType(const nsACString& aMimeType, uint32_t excludeFlags, uint32_t *_retval); \
  nsresult RegisterFakePlugin(JS::HandleValue initDictionary, JSContext* cx, nsIFakePluginTag **_retval); \
  nsresult CreateFakePlugin(JS::HandleValue initDictionary, JSContext* cx, nsIFakePluginTag **_retval); \
  nsresult GetFakePlugin(const nsACString& mimeType, nsIFakePluginTag **_retval); \
  nsresult UnregisterFakePlugin(const nsACString& handlerURI); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPLUGINHOST(_to) \
  NS_IMETHOD ReloadPlugins(void) override { return _to ReloadPlugins(); } \
  NS_IMETHOD GetPluginTags(nsTArray<RefPtr<nsIPluginTag>>& _retval) override { return _to GetPluginTags(_retval); } \
  NS_IMETHOD ClearSiteData(nsIPluginTag *plugin, const nsACString& domain, uint64_t flags, int64_t maxAge, nsIClearSiteDataCallback *callback) override { return _to ClearSiteData(plugin, domain, flags, maxAge, callback); } \
  NS_IMETHOD SiteHasData(nsIPluginTag *plugin, const nsACString& domain, bool *_retval) override { return _to SiteHasData(plugin, domain, _retval); } \
  NS_IMETHOD GetPermissionStringForType(const nsACString& mimeType, uint32_t excludeFlags, nsACString& _retval) override { return _to GetPermissionStringForType(mimeType, excludeFlags, _retval); } \
  NS_IMETHOD GetPermissionStringForTag(nsIPluginTag *tag, uint32_t excludeFlags, nsACString& _retval) override { return _to GetPermissionStringForTag(tag, excludeFlags, _retval); } \
  NS_IMETHOD GetPluginTagForType(const nsACString& mimeType, uint32_t excludeFlags, nsIPluginTag **_retval) override { return _to GetPluginTagForType(mimeType, excludeFlags, _retval); } \
  NS_IMETHOD GetStateForType(const nsACString& mimeType, uint32_t excludeFlags, uint32_t *_retval) override { return _to GetStateForType(mimeType, excludeFlags, _retval); } \
  NS_IMETHOD GetBlocklistStateForType(const nsACString& aMimeType, uint32_t excludeFlags, uint32_t *_retval) override { return _to GetBlocklistStateForType(aMimeType, excludeFlags, _retval); } \
  NS_IMETHOD RegisterFakePlugin(JS::HandleValue initDictionary, JSContext* cx, nsIFakePluginTag **_retval) override { return _to RegisterFakePlugin(initDictionary, cx, _retval); } \
  NS_IMETHOD CreateFakePlugin(JS::HandleValue initDictionary, JSContext* cx, nsIFakePluginTag **_retval) override { return _to CreateFakePlugin(initDictionary, cx, _retval); } \
  NS_IMETHOD GetFakePlugin(const nsACString& mimeType, nsIFakePluginTag **_retval) override { return _to GetFakePlugin(mimeType, _retval); } \
  NS_IMETHOD UnregisterFakePlugin(const nsACString& handlerURI) override { return _to UnregisterFakePlugin(handlerURI); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPLUGINHOST(_to) \
  NS_IMETHOD ReloadPlugins(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReloadPlugins(); } \
  NS_IMETHOD GetPluginTags(nsTArray<RefPtr<nsIPluginTag>>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPluginTags(_retval); } \
  NS_IMETHOD ClearSiteData(nsIPluginTag *plugin, const nsACString& domain, uint64_t flags, int64_t maxAge, nsIClearSiteDataCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearSiteData(plugin, domain, flags, maxAge, callback); } \
  NS_IMETHOD SiteHasData(nsIPluginTag *plugin, const nsACString& domain, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SiteHasData(plugin, domain, _retval); } \
  NS_IMETHOD GetPermissionStringForType(const nsACString& mimeType, uint32_t excludeFlags, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPermissionStringForType(mimeType, excludeFlags, _retval); } \
  NS_IMETHOD GetPermissionStringForTag(nsIPluginTag *tag, uint32_t excludeFlags, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPermissionStringForTag(tag, excludeFlags, _retval); } \
  NS_IMETHOD GetPluginTagForType(const nsACString& mimeType, uint32_t excludeFlags, nsIPluginTag **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPluginTagForType(mimeType, excludeFlags, _retval); } \
  NS_IMETHOD GetStateForType(const nsACString& mimeType, uint32_t excludeFlags, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStateForType(mimeType, excludeFlags, _retval); } \
  NS_IMETHOD GetBlocklistStateForType(const nsACString& aMimeType, uint32_t excludeFlags, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBlocklistStateForType(aMimeType, excludeFlags, _retval); } \
  NS_IMETHOD RegisterFakePlugin(JS::HandleValue initDictionary, JSContext* cx, nsIFakePluginTag **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterFakePlugin(initDictionary, cx, _retval); } \
  NS_IMETHOD CreateFakePlugin(JS::HandleValue initDictionary, JSContext* cx, nsIFakePluginTag **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateFakePlugin(initDictionary, cx, _retval); } \
  NS_IMETHOD GetFakePlugin(const nsACString& mimeType, nsIFakePluginTag **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFakePlugin(mimeType, _retval); } \
  NS_IMETHOD UnregisterFakePlugin(const nsACString& handlerURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnregisterFakePlugin(handlerURI); } 


#endif /* __gen_nsIPluginHost_h__ */
