/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/mime/nsIMIMEInfo.idl
 */

#ifndef __gen_nsIMIMEInfo_h__
#define __gen_nsIMIMEInfo_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsIFile; /* forward declaration */

class nsIUTF8StringEnumerator; /* forward declaration */

class nsIHandlerApp; /* forward declaration */

class nsIArray; /* forward declaration */

class nsIMutableArray; /* forward declaration */

class nsIInterfaceRequestor; /* forward declaration */

namespace mozilla {
namespace dom {
class BrowsingContext; /* webidl BrowsingContext */
} // namespace dom
} // namespace mozilla

typedef int32_t  nsHandlerInfoAction;


/* starting interface:    nsIHandlerInfo */
#define NS_IHANDLERINFO_IID_STR "325e56a7-3762-4312-aec7-f1fcf84b4145"

#define NS_IHANDLERINFO_IID \
  {0x325e56a7, 0x3762, 0x4312, \
    { 0xae, 0xc7, 0xf1, 0xfc, 0xf8, 0x4b, 0x41, 0x45 }}

class NS_NO_VTABLE nsIHandlerInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHANDLERINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIHandlerInfo;

  /* readonly attribute ACString type; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetType(nsACString& aType) = 0;

  /* attribute AString description; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDescription(nsAString& aDescription) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDescription(const nsAString& aDescription) = 0;

  /* attribute nsIHandlerApp preferredApplicationHandler; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPreferredApplicationHandler(nsIHandlerApp **aPreferredApplicationHandler) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPreferredApplicationHandler(nsIHandlerApp *aPreferredApplicationHandler) = 0;

  /* readonly attribute nsIMutableArray possibleApplicationHandlers; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPossibleApplicationHandlers(nsIMutableArray **aPossibleApplicationHandlers) = 0;

  /* readonly attribute boolean hasDefaultHandler; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHasDefaultHandler(bool *aHasDefaultHandler) = 0;

  /* readonly attribute AString defaultDescription; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDefaultDescription(nsAString& aDefaultDescription) = 0;

  /* void launchWithURI (in nsIURI aURI, [optional] in BrowsingContext aBrowsingContext); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD LaunchWithURI(nsIURI *aURI, mozilla::dom::BrowsingContext *aBrowsingContext) = 0;

  /* attribute nsHandlerInfoAction preferredAction; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPreferredAction(nsHandlerInfoAction *aPreferredAction) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPreferredAction(nsHandlerInfoAction aPreferredAction) = 0;

  enum {
    saveToDisk = 0,
    alwaysAsk = 1,
    useHelperApp = 2,
    handleInternally = 3,
    useSystemDefault = 4
  };

  /* attribute boolean alwaysAskBeforeHandling; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAlwaysAskBeforeHandling(bool *aAlwaysAskBeforeHandling) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetAlwaysAskBeforeHandling(bool aAlwaysAskBeforeHandling) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHandlerInfo, NS_IHANDLERINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHANDLERINFO \
  NS_IMETHOD GetType(nsACString& aType) override; \
  NS_IMETHOD GetDescription(nsAString& aDescription) override; \
  NS_IMETHOD SetDescription(const nsAString& aDescription) override; \
  NS_IMETHOD GetPreferredApplicationHandler(nsIHandlerApp **aPreferredApplicationHandler) override; \
  NS_IMETHOD SetPreferredApplicationHandler(nsIHandlerApp *aPreferredApplicationHandler) override; \
  NS_IMETHOD GetPossibleApplicationHandlers(nsIMutableArray **aPossibleApplicationHandlers) override; \
  NS_IMETHOD GetHasDefaultHandler(bool *aHasDefaultHandler) override; \
  NS_IMETHOD GetDefaultDescription(nsAString& aDefaultDescription) override; \
  NS_IMETHOD LaunchWithURI(nsIURI *aURI, mozilla::dom::BrowsingContext *aBrowsingContext) override; \
  NS_IMETHOD GetPreferredAction(nsHandlerInfoAction *aPreferredAction) override; \
  NS_IMETHOD SetPreferredAction(nsHandlerInfoAction aPreferredAction) override; \
  NS_IMETHOD GetAlwaysAskBeforeHandling(bool *aAlwaysAskBeforeHandling) override; \
  NS_IMETHOD SetAlwaysAskBeforeHandling(bool aAlwaysAskBeforeHandling) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHANDLERINFO \
  nsresult GetType(nsACString& aType); \
  nsresult GetDescription(nsAString& aDescription); \
  nsresult SetDescription(const nsAString& aDescription); \
  nsresult GetPreferredApplicationHandler(nsIHandlerApp **aPreferredApplicationHandler); \
  nsresult SetPreferredApplicationHandler(nsIHandlerApp *aPreferredApplicationHandler); \
  nsresult GetPossibleApplicationHandlers(nsIMutableArray **aPossibleApplicationHandlers); \
  nsresult GetHasDefaultHandler(bool *aHasDefaultHandler); \
  nsresult GetDefaultDescription(nsAString& aDefaultDescription); \
  nsresult LaunchWithURI(nsIURI *aURI, mozilla::dom::BrowsingContext *aBrowsingContext); \
  nsresult GetPreferredAction(nsHandlerInfoAction *aPreferredAction); \
  nsresult SetPreferredAction(nsHandlerInfoAction aPreferredAction); \
  nsresult GetAlwaysAskBeforeHandling(bool *aAlwaysAskBeforeHandling); \
  nsresult SetAlwaysAskBeforeHandling(bool aAlwaysAskBeforeHandling); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHANDLERINFO(_to) \
  NS_IMETHOD GetType(nsACString& aType) override { return _to GetType(aType); } \
  NS_IMETHOD GetDescription(nsAString& aDescription) override { return _to GetDescription(aDescription); } \
  NS_IMETHOD SetDescription(const nsAString& aDescription) override { return _to SetDescription(aDescription); } \
  NS_IMETHOD GetPreferredApplicationHandler(nsIHandlerApp **aPreferredApplicationHandler) override { return _to GetPreferredApplicationHandler(aPreferredApplicationHandler); } \
  NS_IMETHOD SetPreferredApplicationHandler(nsIHandlerApp *aPreferredApplicationHandler) override { return _to SetPreferredApplicationHandler(aPreferredApplicationHandler); } \
  NS_IMETHOD GetPossibleApplicationHandlers(nsIMutableArray **aPossibleApplicationHandlers) override { return _to GetPossibleApplicationHandlers(aPossibleApplicationHandlers); } \
  NS_IMETHOD GetHasDefaultHandler(bool *aHasDefaultHandler) override { return _to GetHasDefaultHandler(aHasDefaultHandler); } \
  NS_IMETHOD GetDefaultDescription(nsAString& aDefaultDescription) override { return _to GetDefaultDescription(aDefaultDescription); } \
  NS_IMETHOD LaunchWithURI(nsIURI *aURI, mozilla::dom::BrowsingContext *aBrowsingContext) override { return _to LaunchWithURI(aURI, aBrowsingContext); } \
  NS_IMETHOD GetPreferredAction(nsHandlerInfoAction *aPreferredAction) override { return _to GetPreferredAction(aPreferredAction); } \
  NS_IMETHOD SetPreferredAction(nsHandlerInfoAction aPreferredAction) override { return _to SetPreferredAction(aPreferredAction); } \
  NS_IMETHOD GetAlwaysAskBeforeHandling(bool *aAlwaysAskBeforeHandling) override { return _to GetAlwaysAskBeforeHandling(aAlwaysAskBeforeHandling); } \
  NS_IMETHOD SetAlwaysAskBeforeHandling(bool aAlwaysAskBeforeHandling) override { return _to SetAlwaysAskBeforeHandling(aAlwaysAskBeforeHandling); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHANDLERINFO(_to) \
  NS_IMETHOD GetType(nsACString& aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetType(aType); } \
  NS_IMETHOD GetDescription(nsAString& aDescription) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDescription(aDescription); } \
  NS_IMETHOD SetDescription(const nsAString& aDescription) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDescription(aDescription); } \
  NS_IMETHOD GetPreferredApplicationHandler(nsIHandlerApp **aPreferredApplicationHandler) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPreferredApplicationHandler(aPreferredApplicationHandler); } \
  NS_IMETHOD SetPreferredApplicationHandler(nsIHandlerApp *aPreferredApplicationHandler) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPreferredApplicationHandler(aPreferredApplicationHandler); } \
  NS_IMETHOD GetPossibleApplicationHandlers(nsIMutableArray **aPossibleApplicationHandlers) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPossibleApplicationHandlers(aPossibleApplicationHandlers); } \
  NS_IMETHOD GetHasDefaultHandler(bool *aHasDefaultHandler) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHasDefaultHandler(aHasDefaultHandler); } \
  NS_IMETHOD GetDefaultDescription(nsAString& aDefaultDescription) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefaultDescription(aDefaultDescription); } \
  NS_IMETHOD LaunchWithURI(nsIURI *aURI, mozilla::dom::BrowsingContext *aBrowsingContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LaunchWithURI(aURI, aBrowsingContext); } \
  NS_IMETHOD GetPreferredAction(nsHandlerInfoAction *aPreferredAction) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPreferredAction(aPreferredAction); } \
  NS_IMETHOD SetPreferredAction(nsHandlerInfoAction aPreferredAction) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPreferredAction(aPreferredAction); } \
  NS_IMETHOD GetAlwaysAskBeforeHandling(bool *aAlwaysAskBeforeHandling) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAlwaysAskBeforeHandling(aAlwaysAskBeforeHandling); } \
  NS_IMETHOD SetAlwaysAskBeforeHandling(bool aAlwaysAskBeforeHandling) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAlwaysAskBeforeHandling(aAlwaysAskBeforeHandling); } 


/* starting interface:    nsIMIMEInfo */
#define NS_IMIMEINFO_IID_STR "1c21acef-c7a1-40c6-9d40-a20480ee53a1"

#define NS_IMIMEINFO_IID \
  {0x1c21acef, 0xc7a1, 0x40c6, \
    { 0x9d, 0x40, 0xa2, 0x04, 0x80, 0xee, 0x53, 0xa1 }}

class NS_NO_VTABLE nsIMIMEInfo : public nsIHandlerInfo {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IMIMEINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIMIMEInfo;

  /* nsIUTF8StringEnumerator getFileExtensions (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFileExtensions(nsIUTF8StringEnumerator **_retval) = 0;

  /* void setFileExtensions (in AUTF8String aExtensions); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetFileExtensions(const nsACString& aExtensions) = 0;

  /* boolean extensionExists (in AUTF8String aExtension); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ExtensionExists(const nsACString& aExtension, bool *_retval) = 0;

  /* void appendExtension (in AUTF8String aExtension); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AppendExtension(const nsACString& aExtension) = 0;

  /* attribute AUTF8String primaryExtension; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPrimaryExtension(nsACString& aPrimaryExtension) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPrimaryExtension(const nsACString& aPrimaryExtension) = 0;

  /* readonly attribute ACString MIMEType; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMIMEType(nsACString& aMIMEType) = 0;

  /* boolean equals (in nsIMIMEInfo aMIMEInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Equals(nsIMIMEInfo *aMIMEInfo, bool *_retval) = 0;

  /* readonly attribute nsIArray possibleLocalHandlers; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPossibleLocalHandlers(nsIArray **aPossibleLocalHandlers) = 0;

  /* void launchWithFile (in nsIFile aFile); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD LaunchWithFile(nsIFile *aFile) = 0;

  /* boolean isCurrentAppOSDefault (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsCurrentAppOSDefault(bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIMIMEInfo, NS_IMIMEINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIMIMEINFO \
  NS_IMETHOD GetFileExtensions(nsIUTF8StringEnumerator **_retval) override; \
  NS_IMETHOD SetFileExtensions(const nsACString& aExtensions) override; \
  NS_IMETHOD ExtensionExists(const nsACString& aExtension, bool *_retval) override; \
  NS_IMETHOD AppendExtension(const nsACString& aExtension) override; \
  NS_IMETHOD GetPrimaryExtension(nsACString& aPrimaryExtension) override; \
  NS_IMETHOD SetPrimaryExtension(const nsACString& aPrimaryExtension) override; \
  NS_IMETHOD GetMIMEType(nsACString& aMIMEType) override; \
  NS_IMETHOD Equals(nsIMIMEInfo *aMIMEInfo, bool *_retval) override; \
  NS_IMETHOD GetPossibleLocalHandlers(nsIArray **aPossibleLocalHandlers) override; \
  NS_IMETHOD LaunchWithFile(nsIFile *aFile) override; \
  NS_IMETHOD IsCurrentAppOSDefault(bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIMIMEINFO \
  nsresult GetFileExtensions(nsIUTF8StringEnumerator **_retval); \
  nsresult SetFileExtensions(const nsACString& aExtensions); \
  nsresult ExtensionExists(const nsACString& aExtension, bool *_retval); \
  nsresult AppendExtension(const nsACString& aExtension); \
  nsresult GetPrimaryExtension(nsACString& aPrimaryExtension); \
  nsresult SetPrimaryExtension(const nsACString& aPrimaryExtension); \
  nsresult GetMIMEType(nsACString& aMIMEType); \
  nsresult Equals(nsIMIMEInfo *aMIMEInfo, bool *_retval); \
  nsresult GetPossibleLocalHandlers(nsIArray **aPossibleLocalHandlers); \
  nsresult LaunchWithFile(nsIFile *aFile); \
  nsresult IsCurrentAppOSDefault(bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIMIMEINFO(_to) \
  NS_IMETHOD GetFileExtensions(nsIUTF8StringEnumerator **_retval) override { return _to GetFileExtensions(_retval); } \
  NS_IMETHOD SetFileExtensions(const nsACString& aExtensions) override { return _to SetFileExtensions(aExtensions); } \
  NS_IMETHOD ExtensionExists(const nsACString& aExtension, bool *_retval) override { return _to ExtensionExists(aExtension, _retval); } \
  NS_IMETHOD AppendExtension(const nsACString& aExtension) override { return _to AppendExtension(aExtension); } \
  NS_IMETHOD GetPrimaryExtension(nsACString& aPrimaryExtension) override { return _to GetPrimaryExtension(aPrimaryExtension); } \
  NS_IMETHOD SetPrimaryExtension(const nsACString& aPrimaryExtension) override { return _to SetPrimaryExtension(aPrimaryExtension); } \
  NS_IMETHOD GetMIMEType(nsACString& aMIMEType) override { return _to GetMIMEType(aMIMEType); } \
  NS_IMETHOD Equals(nsIMIMEInfo *aMIMEInfo, bool *_retval) override { return _to Equals(aMIMEInfo, _retval); } \
  NS_IMETHOD GetPossibleLocalHandlers(nsIArray **aPossibleLocalHandlers) override { return _to GetPossibleLocalHandlers(aPossibleLocalHandlers); } \
  NS_IMETHOD LaunchWithFile(nsIFile *aFile) override { return _to LaunchWithFile(aFile); } \
  NS_IMETHOD IsCurrentAppOSDefault(bool *_retval) override { return _to IsCurrentAppOSDefault(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIMIMEINFO(_to) \
  NS_IMETHOD GetFileExtensions(nsIUTF8StringEnumerator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFileExtensions(_retval); } \
  NS_IMETHOD SetFileExtensions(const nsACString& aExtensions) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFileExtensions(aExtensions); } \
  NS_IMETHOD ExtensionExists(const nsACString& aExtension, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ExtensionExists(aExtension, _retval); } \
  NS_IMETHOD AppendExtension(const nsACString& aExtension) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AppendExtension(aExtension); } \
  NS_IMETHOD GetPrimaryExtension(nsACString& aPrimaryExtension) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrimaryExtension(aPrimaryExtension); } \
  NS_IMETHOD SetPrimaryExtension(const nsACString& aPrimaryExtension) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPrimaryExtension(aPrimaryExtension); } \
  NS_IMETHOD GetMIMEType(nsACString& aMIMEType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMIMEType(aMIMEType); } \
  NS_IMETHOD Equals(nsIMIMEInfo *aMIMEInfo, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Equals(aMIMEInfo, _retval); } \
  NS_IMETHOD GetPossibleLocalHandlers(nsIArray **aPossibleLocalHandlers) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPossibleLocalHandlers(aPossibleLocalHandlers); } \
  NS_IMETHOD LaunchWithFile(nsIFile *aFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LaunchWithFile(aFile); } \
  NS_IMETHOD IsCurrentAppOSDefault(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsCurrentAppOSDefault(_retval); } 


/* starting interface:    nsIHandlerApp */
#define NS_IHANDLERAPP_IID_STR "8bdf20a4-9170-4548-af52-78311a44f920"

#define NS_IHANDLERAPP_IID \
  {0x8bdf20a4, 0x9170, 0x4548, \
    { 0xaf, 0x52, 0x78, 0x31, 0x1a, 0x44, 0xf9, 0x20 }}

class NS_NO_VTABLE nsIHandlerApp : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHANDLERAPP_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIHandlerApp;

  /* attribute AString name; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetName(nsAString& aName) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetName(const nsAString& aName) = 0;

  /* attribute AString detailedDescription; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDetailedDescription(nsAString& aDetailedDescription) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDetailedDescription(const nsAString& aDetailedDescription) = 0;

  /* boolean equals (in nsIHandlerApp aHandlerApp); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Equals(nsIHandlerApp *aHandlerApp, bool *_retval) = 0;

  /* void launchWithURI (in nsIURI aURI, [optional] in BrowsingContext aBrowsingContext); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD LaunchWithURI(nsIURI *aURI, mozilla::dom::BrowsingContext *aBrowsingContext) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHandlerApp, NS_IHANDLERAPP_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHANDLERAPP \
  NS_IMETHOD GetName(nsAString& aName) override; \
  NS_IMETHOD SetName(const nsAString& aName) override; \
  NS_IMETHOD GetDetailedDescription(nsAString& aDetailedDescription) override; \
  NS_IMETHOD SetDetailedDescription(const nsAString& aDetailedDescription) override; \
  NS_IMETHOD Equals(nsIHandlerApp *aHandlerApp, bool *_retval) override; \
  NS_IMETHOD LaunchWithURI(nsIURI *aURI, mozilla::dom::BrowsingContext *aBrowsingContext) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHANDLERAPP \
  nsresult GetName(nsAString& aName); \
  nsresult SetName(const nsAString& aName); \
  nsresult GetDetailedDescription(nsAString& aDetailedDescription); \
  nsresult SetDetailedDescription(const nsAString& aDetailedDescription); \
  nsresult Equals(nsIHandlerApp *aHandlerApp, bool *_retval); \
  nsresult LaunchWithURI(nsIURI *aURI, mozilla::dom::BrowsingContext *aBrowsingContext); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHANDLERAPP(_to) \
  NS_IMETHOD GetName(nsAString& aName) override { return _to GetName(aName); } \
  NS_IMETHOD SetName(const nsAString& aName) override { return _to SetName(aName); } \
  NS_IMETHOD GetDetailedDescription(nsAString& aDetailedDescription) override { return _to GetDetailedDescription(aDetailedDescription); } \
  NS_IMETHOD SetDetailedDescription(const nsAString& aDetailedDescription) override { return _to SetDetailedDescription(aDetailedDescription); } \
  NS_IMETHOD Equals(nsIHandlerApp *aHandlerApp, bool *_retval) override { return _to Equals(aHandlerApp, _retval); } \
  NS_IMETHOD LaunchWithURI(nsIURI *aURI, mozilla::dom::BrowsingContext *aBrowsingContext) override { return _to LaunchWithURI(aURI, aBrowsingContext); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHANDLERAPP(_to) \
  NS_IMETHOD GetName(nsAString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetName(aName); } \
  NS_IMETHOD SetName(const nsAString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetName(aName); } \
  NS_IMETHOD GetDetailedDescription(nsAString& aDetailedDescription) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDetailedDescription(aDetailedDescription); } \
  NS_IMETHOD SetDetailedDescription(const nsAString& aDetailedDescription) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDetailedDescription(aDetailedDescription); } \
  NS_IMETHOD Equals(nsIHandlerApp *aHandlerApp, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Equals(aHandlerApp, _retval); } \
  NS_IMETHOD LaunchWithURI(nsIURI *aURI, mozilla::dom::BrowsingContext *aBrowsingContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LaunchWithURI(aURI, aBrowsingContext); } 


/* starting interface:    nsILocalHandlerApp */
#define NS_ILOCALHANDLERAPP_IID_STR "d36b6329-52ae-4f45-80f4-b2536ae5f8b2"

#define NS_ILOCALHANDLERAPP_IID \
  {0xd36b6329, 0x52ae, 0x4f45, \
    { 0x80, 0xf4, 0xb2, 0x53, 0x6a, 0xe5, 0xf8, 0xb2 }}

class NS_NO_VTABLE nsILocalHandlerApp : public nsIHandlerApp {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILOCALHANDLERAPP_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsILocalHandlerApp;

  /* attribute nsIFile executable; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetExecutable(nsIFile **aExecutable) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetExecutable(nsIFile *aExecutable) = 0;

  /* readonly attribute unsigned long parameterCount; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetParameterCount(uint32_t *aParameterCount) = 0;

  /* void clearParameters (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ClearParameters(void) = 0;

  /* void appendParameter (in AString param); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AppendParameter(const nsAString& param) = 0;

  /* AString getParameter (in unsigned long parameterIndex); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetParameter(uint32_t parameterIndex, nsAString& _retval) = 0;

  /* boolean parameterExists (in AString param); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ParameterExists(const nsAString& param, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsILocalHandlerApp, NS_ILOCALHANDLERAPP_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILOCALHANDLERAPP \
  NS_IMETHOD GetExecutable(nsIFile **aExecutable) override; \
  NS_IMETHOD SetExecutable(nsIFile *aExecutable) override; \
  NS_IMETHOD GetParameterCount(uint32_t *aParameterCount) override; \
  NS_IMETHOD ClearParameters(void) override; \
  NS_IMETHOD AppendParameter(const nsAString& param) override; \
  NS_IMETHOD GetParameter(uint32_t parameterIndex, nsAString& _retval) override; \
  NS_IMETHOD ParameterExists(const nsAString& param, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILOCALHANDLERAPP \
  nsresult GetExecutable(nsIFile **aExecutable); \
  nsresult SetExecutable(nsIFile *aExecutable); \
  nsresult GetParameterCount(uint32_t *aParameterCount); \
  nsresult ClearParameters(void); \
  nsresult AppendParameter(const nsAString& param); \
  nsresult GetParameter(uint32_t parameterIndex, nsAString& _retval); \
  nsresult ParameterExists(const nsAString& param, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILOCALHANDLERAPP(_to) \
  NS_IMETHOD GetExecutable(nsIFile **aExecutable) override { return _to GetExecutable(aExecutable); } \
  NS_IMETHOD SetExecutable(nsIFile *aExecutable) override { return _to SetExecutable(aExecutable); } \
  NS_IMETHOD GetParameterCount(uint32_t *aParameterCount) override { return _to GetParameterCount(aParameterCount); } \
  NS_IMETHOD ClearParameters(void) override { return _to ClearParameters(); } \
  NS_IMETHOD AppendParameter(const nsAString& param) override { return _to AppendParameter(param); } \
  NS_IMETHOD GetParameter(uint32_t parameterIndex, nsAString& _retval) override { return _to GetParameter(parameterIndex, _retval); } \
  NS_IMETHOD ParameterExists(const nsAString& param, bool *_retval) override { return _to ParameterExists(param, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILOCALHANDLERAPP(_to) \
  NS_IMETHOD GetExecutable(nsIFile **aExecutable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExecutable(aExecutable); } \
  NS_IMETHOD SetExecutable(nsIFile *aExecutable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetExecutable(aExecutable); } \
  NS_IMETHOD GetParameterCount(uint32_t *aParameterCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetParameterCount(aParameterCount); } \
  NS_IMETHOD ClearParameters(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearParameters(); } \
  NS_IMETHOD AppendParameter(const nsAString& param) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AppendParameter(param); } \
  NS_IMETHOD GetParameter(uint32_t parameterIndex, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetParameter(parameterIndex, _retval); } \
  NS_IMETHOD ParameterExists(const nsAString& param, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ParameterExists(param, _retval); } 


/* starting interface:    nsIWebHandlerApp */
#define NS_IWEBHANDLERAPP_IID_STR "7521a093-c498-45ce-b462-df7ba0d882f6"

#define NS_IWEBHANDLERAPP_IID \
  {0x7521a093, 0xc498, 0x45ce, \
    { 0xb4, 0x62, 0xdf, 0x7b, 0xa0, 0xd8, 0x82, 0xf6 }}

class NS_NO_VTABLE nsIWebHandlerApp : public nsIHandlerApp {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWEBHANDLERAPP_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWebHandlerApp;

  /* attribute AUTF8String uriTemplate; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUriTemplate(nsACString& aUriTemplate) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetUriTemplate(const nsACString& aUriTemplate) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWebHandlerApp, NS_IWEBHANDLERAPP_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWEBHANDLERAPP \
  NS_IMETHOD GetUriTemplate(nsACString& aUriTemplate) override; \
  NS_IMETHOD SetUriTemplate(const nsACString& aUriTemplate) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWEBHANDLERAPP \
  nsresult GetUriTemplate(nsACString& aUriTemplate); \
  nsresult SetUriTemplate(const nsACString& aUriTemplate); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWEBHANDLERAPP(_to) \
  NS_IMETHOD GetUriTemplate(nsACString& aUriTemplate) override { return _to GetUriTemplate(aUriTemplate); } \
  NS_IMETHOD SetUriTemplate(const nsACString& aUriTemplate) override { return _to SetUriTemplate(aUriTemplate); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWEBHANDLERAPP(_to) \
  NS_IMETHOD GetUriTemplate(nsACString& aUriTemplate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUriTemplate(aUriTemplate); } \
  NS_IMETHOD SetUriTemplate(const nsACString& aUriTemplate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetUriTemplate(aUriTemplate); } 


/* starting interface:    nsIDBusHandlerApp */
#define NS_IDBUSHANDLERAPP_IID_STR "1ffc274b-4cbf-4bb5-a635-05ad2cbb6534"

#define NS_IDBUSHANDLERAPP_IID \
  {0x1ffc274b, 0x4cbf, 0x4bb5, \
    { 0xa6, 0x35, 0x05, 0xad, 0x2c, 0xbb, 0x65, 0x34 }}

class NS_NO_VTABLE nsIDBusHandlerApp : public nsIHandlerApp {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDBUSHANDLERAPP_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDBusHandlerApp;

  /* attribute AUTF8String service; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetService(nsACString& aService) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetService(const nsACString& aService) = 0;

  /* attribute AUTF8String objectPath; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetObjectPath(nsACString& aObjectPath) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetObjectPath(const nsACString& aObjectPath) = 0;

  /* attribute AUTF8String dBusInterface; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDBusInterface(nsACString& aDBusInterface) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDBusInterface(const nsACString& aDBusInterface) = 0;

  /* attribute AUTF8String method; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMethod(nsACString& aMethod) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetMethod(const nsACString& aMethod) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDBusHandlerApp, NS_IDBUSHANDLERAPP_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDBUSHANDLERAPP \
  NS_IMETHOD GetService(nsACString& aService) override; \
  NS_IMETHOD SetService(const nsACString& aService) override; \
  NS_IMETHOD GetObjectPath(nsACString& aObjectPath) override; \
  NS_IMETHOD SetObjectPath(const nsACString& aObjectPath) override; \
  NS_IMETHOD GetDBusInterface(nsACString& aDBusInterface) override; \
  NS_IMETHOD SetDBusInterface(const nsACString& aDBusInterface) override; \
  NS_IMETHOD GetMethod(nsACString& aMethod) override; \
  NS_IMETHOD SetMethod(const nsACString& aMethod) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDBUSHANDLERAPP \
  nsresult GetService(nsACString& aService); \
  nsresult SetService(const nsACString& aService); \
  nsresult GetObjectPath(nsACString& aObjectPath); \
  nsresult SetObjectPath(const nsACString& aObjectPath); \
  nsresult GetDBusInterface(nsACString& aDBusInterface); \
  nsresult SetDBusInterface(const nsACString& aDBusInterface); \
  nsresult GetMethod(nsACString& aMethod); \
  nsresult SetMethod(const nsACString& aMethod); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDBUSHANDLERAPP(_to) \
  NS_IMETHOD GetService(nsACString& aService) override { return _to GetService(aService); } \
  NS_IMETHOD SetService(const nsACString& aService) override { return _to SetService(aService); } \
  NS_IMETHOD GetObjectPath(nsACString& aObjectPath) override { return _to GetObjectPath(aObjectPath); } \
  NS_IMETHOD SetObjectPath(const nsACString& aObjectPath) override { return _to SetObjectPath(aObjectPath); } \
  NS_IMETHOD GetDBusInterface(nsACString& aDBusInterface) override { return _to GetDBusInterface(aDBusInterface); } \
  NS_IMETHOD SetDBusInterface(const nsACString& aDBusInterface) override { return _to SetDBusInterface(aDBusInterface); } \
  NS_IMETHOD GetMethod(nsACString& aMethod) override { return _to GetMethod(aMethod); } \
  NS_IMETHOD SetMethod(const nsACString& aMethod) override { return _to SetMethod(aMethod); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDBUSHANDLERAPP(_to) \
  NS_IMETHOD GetService(nsACString& aService) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetService(aService); } \
  NS_IMETHOD SetService(const nsACString& aService) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetService(aService); } \
  NS_IMETHOD GetObjectPath(nsACString& aObjectPath) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetObjectPath(aObjectPath); } \
  NS_IMETHOD SetObjectPath(const nsACString& aObjectPath) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetObjectPath(aObjectPath); } \
  NS_IMETHOD GetDBusInterface(nsACString& aDBusInterface) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDBusInterface(aDBusInterface); } \
  NS_IMETHOD SetDBusInterface(const nsACString& aDBusInterface) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDBusInterface(aDBusInterface); } \
  NS_IMETHOD GetMethod(nsACString& aMethod) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMethod(aMethod); } \
  NS_IMETHOD SetMethod(const nsACString& aMethod) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMethod(aMethod); } 


#endif /* __gen_nsIMIMEInfo_h__ */
