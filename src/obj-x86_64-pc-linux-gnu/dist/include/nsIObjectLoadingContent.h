/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsIObjectLoadingContent.idl
 */

#ifndef __gen_nsIObjectLoadingContent_h__
#define __gen_nsIObjectLoadingContent_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIChannel; /* forward declaration */

class nsIRequest; /* forward declaration */

class nsIFrame; /* forward declaration */

class nsIPluginTag; /* forward declaration */

class nsIURI; /* forward declaration */

namespace mozilla {
namespace dom {
class BrowsingContext; /* webidl BrowsingContext */
} // namespace dom
} // namespace mozilla

class nsNPAPIPluginInstance;

/* starting interface:    nsIObjectLoadingContent */
#define NS_IOBJECTLOADINGCONTENT_IID_STR "2eb3195e-3eea-4083-bb1d-d2d70fa35ccb"

#define NS_IOBJECTLOADINGCONTENT_IID \
  {0x2eb3195e, 0x3eea, 0x4083, \
    { 0xbb, 0x1d, 0xd2, 0xd7, 0x0f, 0xa3, 0x5c, 0xcb }}

class NS_NO_VTABLE nsIObjectLoadingContent : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IOBJECTLOADINGCONTENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIObjectLoadingContent;

  enum {
    TYPE_LOADING = 0U,
    TYPE_IMAGE = 1U,
    TYPE_PLUGIN = 2U,
    TYPE_FAKE_PLUGIN = 3U,
    TYPE_DOCUMENT = 4U,
    TYPE_NULL = 5U,
    PLUGIN_ACTIVE = 255U,
    PLUGIN_UNSUPPORTED = 0U,
    PLUGIN_ALTERNATE = 1U,
    PLUGIN_DISABLED = 2U,
    PLUGIN_BLOCKLISTED = 3U,
    PLUGIN_OUTDATED = 4U,
    PLUGIN_CRASHED = 5U,
    PLUGIN_CLICK_TO_PLAY = 8U,
    PLUGIN_VULNERABLE_UPDATABLE = 9U,
    PLUGIN_VULNERABLE_NO_UPDATE = 10U,
    PLUGIN_CLICK_TO_PLAY_QUIET = 11U,
    PLUGIN_BLOCK_ALL = 12U,
    PLUGIN_PERMISSION_PROMPT_ACTION_QUIET = 8U
  };

  /* readonly attribute ACString actualType; */
  NS_IMETHOD GetActualType(nsACString& aActualType) = 0;

  /* readonly attribute unsigned long displayedType; */
  NS_IMETHOD GetDisplayedType(uint32_t *aDisplayedType) = 0;

  /* unsigned long getContentTypeForMIMEType (in AUTF8String aMimeType); */
  NS_IMETHOD GetContentTypeForMIMEType(const nsACString& aMimeType, uint32_t *_retval) = 0;

  /* [nostdcall,notxpcom] readonly attribute nsNPAPIPluginInstancePtr pluginInstance; */
  virtual nsNPAPIPluginInstance * GetPluginInstance() = 0;

  /* [noscript] void pluginDestroyed (); */
  NS_IMETHOD PluginDestroyed(void) = 0;

  /* [noscript] void pluginCrashed (in nsIPluginTag pluginTag, in AString pluginDumpID, in boolean submittedCrashReport); */
  NS_IMETHOD PluginCrashed(nsIPluginTag *pluginTag, const nsAString& pluginDumpID, bool submittedCrashReport) = 0;

  /* void reload (in boolean aClearActivation); */
  NS_IMETHOD Reload(bool aClearActivation) = 0;

  /* readonly attribute boolean activated; */
  NS_IMETHOD GetActivated(bool *aActivated) = 0;

  /* [noscript] void stopPluginInstance (); */
  NS_IMETHOD StopPluginInstance(void) = 0;

  /* [noscript] void syncStartPluginInstance (); */
  NS_IMETHOD SyncStartPluginInstance(void) = 0;

  /* [noscript] void asyncStartPluginInstance (); */
  NS_IMETHOD AsyncStartPluginInstance(void) = 0;

  /* [noscript] void initializeFromChannel (in nsIRequest request); */
  NS_IMETHOD InitializeFromChannel(nsIRequest *request) = 0;

  /* readonly attribute nsIURI srcURI; */
  NS_IMETHOD GetSrcURI(nsIURI **aSrcURI) = 0;

  /* void skipFakePlugins (); */
  NS_IMETHOD SkipFakePlugins(void) = 0;

  /* [noscript] BrowsingContext upgradeLoadToDocument (in nsIChannel channel); */
  NS_IMETHOD UpgradeLoadToDocument(nsIChannel *channel, mozilla::dom::BrowsingContext **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIObjectLoadingContent, NS_IOBJECTLOADINGCONTENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIOBJECTLOADINGCONTENT \
  NS_IMETHOD GetActualType(nsACString& aActualType) override; \
  NS_IMETHOD GetDisplayedType(uint32_t *aDisplayedType) override; \
  NS_IMETHOD GetContentTypeForMIMEType(const nsACString& aMimeType, uint32_t *_retval) override; \
  virtual nsNPAPIPluginInstance * GetPluginInstance() override; \
  NS_IMETHOD PluginDestroyed(void) override; \
  NS_IMETHOD PluginCrashed(nsIPluginTag *pluginTag, const nsAString& pluginDumpID, bool submittedCrashReport) override; \
  NS_IMETHOD Reload(bool aClearActivation) override; \
  NS_IMETHOD GetActivated(bool *aActivated) override; \
  NS_IMETHOD StopPluginInstance(void) override; \
  NS_IMETHOD SyncStartPluginInstance(void) override; \
  NS_IMETHOD AsyncStartPluginInstance(void) override; \
  NS_IMETHOD InitializeFromChannel(nsIRequest *request) override; \
  NS_IMETHOD GetSrcURI(nsIURI **aSrcURI) override; \
  NS_IMETHOD SkipFakePlugins(void) override; \
  NS_IMETHOD UpgradeLoadToDocument(nsIChannel *channel, mozilla::dom::BrowsingContext **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIOBJECTLOADINGCONTENT \
  nsresult GetActualType(nsACString& aActualType); \
  nsresult GetDisplayedType(uint32_t *aDisplayedType); \
  nsresult GetContentTypeForMIMEType(const nsACString& aMimeType, uint32_t *_retval); \
  nsNPAPIPluginInstance * GetPluginInstance(); \
  nsresult PluginDestroyed(void); \
  nsresult PluginCrashed(nsIPluginTag *pluginTag, const nsAString& pluginDumpID, bool submittedCrashReport); \
  nsresult Reload(bool aClearActivation); \
  nsresult GetActivated(bool *aActivated); \
  nsresult StopPluginInstance(void); \
  nsresult SyncStartPluginInstance(void); \
  nsresult AsyncStartPluginInstance(void); \
  nsresult InitializeFromChannel(nsIRequest *request); \
  nsresult GetSrcURI(nsIURI **aSrcURI); \
  nsresult SkipFakePlugins(void); \
  nsresult UpgradeLoadToDocument(nsIChannel *channel, mozilla::dom::BrowsingContext **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIOBJECTLOADINGCONTENT(_to) \
  NS_IMETHOD GetActualType(nsACString& aActualType) override { return _to GetActualType(aActualType); } \
  NS_IMETHOD GetDisplayedType(uint32_t *aDisplayedType) override { return _to GetDisplayedType(aDisplayedType); } \
  NS_IMETHOD GetContentTypeForMIMEType(const nsACString& aMimeType, uint32_t *_retval) override { return _to GetContentTypeForMIMEType(aMimeType, _retval); } \
  virtual nsNPAPIPluginInstance * GetPluginInstance() override { return _to GetPluginInstance(); } \
  NS_IMETHOD PluginDestroyed(void) override { return _to PluginDestroyed(); } \
  NS_IMETHOD PluginCrashed(nsIPluginTag *pluginTag, const nsAString& pluginDumpID, bool submittedCrashReport) override { return _to PluginCrashed(pluginTag, pluginDumpID, submittedCrashReport); } \
  NS_IMETHOD Reload(bool aClearActivation) override { return _to Reload(aClearActivation); } \
  NS_IMETHOD GetActivated(bool *aActivated) override { return _to GetActivated(aActivated); } \
  NS_IMETHOD StopPluginInstance(void) override { return _to StopPluginInstance(); } \
  NS_IMETHOD SyncStartPluginInstance(void) override { return _to SyncStartPluginInstance(); } \
  NS_IMETHOD AsyncStartPluginInstance(void) override { return _to AsyncStartPluginInstance(); } \
  NS_IMETHOD InitializeFromChannel(nsIRequest *request) override { return _to InitializeFromChannel(request); } \
  NS_IMETHOD GetSrcURI(nsIURI **aSrcURI) override { return _to GetSrcURI(aSrcURI); } \
  NS_IMETHOD SkipFakePlugins(void) override { return _to SkipFakePlugins(); } \
  NS_IMETHOD UpgradeLoadToDocument(nsIChannel *channel, mozilla::dom::BrowsingContext **_retval) override { return _to UpgradeLoadToDocument(channel, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIOBJECTLOADINGCONTENT(_to) \
  NS_IMETHOD GetActualType(nsACString& aActualType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetActualType(aActualType); } \
  NS_IMETHOD GetDisplayedType(uint32_t *aDisplayedType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDisplayedType(aDisplayedType); } \
  NS_IMETHOD GetContentTypeForMIMEType(const nsACString& aMimeType, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentTypeForMIMEType(aMimeType, _retval); } \
  virtual nsNPAPIPluginInstance * GetPluginInstance() override; \
  NS_IMETHOD PluginDestroyed(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PluginDestroyed(); } \
  NS_IMETHOD PluginCrashed(nsIPluginTag *pluginTag, const nsAString& pluginDumpID, bool submittedCrashReport) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PluginCrashed(pluginTag, pluginDumpID, submittedCrashReport); } \
  NS_IMETHOD Reload(bool aClearActivation) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Reload(aClearActivation); } \
  NS_IMETHOD GetActivated(bool *aActivated) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetActivated(aActivated); } \
  NS_IMETHOD StopPluginInstance(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StopPluginInstance(); } \
  NS_IMETHOD SyncStartPluginInstance(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SyncStartPluginInstance(); } \
  NS_IMETHOD AsyncStartPluginInstance(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncStartPluginInstance(); } \
  NS_IMETHOD InitializeFromChannel(nsIRequest *request) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitializeFromChannel(request); } \
  NS_IMETHOD GetSrcURI(nsIURI **aSrcURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSrcURI(aSrcURI); } \
  NS_IMETHOD SkipFakePlugins(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SkipFakePlugins(); } \
  NS_IMETHOD UpgradeLoadToDocument(nsIChannel *channel, mozilla::dom::BrowsingContext **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpgradeLoadToDocument(channel, _retval); } 


#endif /* __gen_nsIObjectLoadingContent_h__ */
