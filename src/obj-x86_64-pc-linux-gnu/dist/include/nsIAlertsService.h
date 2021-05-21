/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/alerts/nsIAlertsService.idl
 */

#ifndef __gen_nsIAlertsService_h__
#define __gen_nsIAlertsService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIObserver_h__
#include "nsIObserver.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class imgIRequest; /* forward declaration */

class nsICancelable; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class nsIURI; /* forward declaration */

#define ALERT_NOTIFICATION_CONTRACTID "@mozilla.org/alert-notification;1"

/* starting interface:    nsIAlertNotificationImageListener */
#define NS_IALERTNOTIFICATIONIMAGELISTENER_IID_STR "a71a637d-de1d-47c6-a8d2-c60b2596f471"

#define NS_IALERTNOTIFICATIONIMAGELISTENER_IID \
  {0xa71a637d, 0xde1d, 0x47c6, \
    { 0xa8, 0xd2, 0xc6, 0x0b, 0x25, 0x96, 0xf4, 0x71 }}

class NS_NO_VTABLE nsIAlertNotificationImageListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IALERTNOTIFICATIONIMAGELISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAlertNotificationImageListener;

  /* void onImageReady (in nsISupports aUserData, in imgIRequest aRequest); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnImageReady(nsISupports *aUserData, imgIRequest *aRequest) = 0;

  /* void onImageMissing (in nsISupports aUserData); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnImageMissing(nsISupports *aUserData) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAlertNotificationImageListener, NS_IALERTNOTIFICATIONIMAGELISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIALERTNOTIFICATIONIMAGELISTENER \
  NS_IMETHOD OnImageReady(nsISupports *aUserData, imgIRequest *aRequest) override; \
  NS_IMETHOD OnImageMissing(nsISupports *aUserData) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIALERTNOTIFICATIONIMAGELISTENER \
  nsresult OnImageReady(nsISupports *aUserData, imgIRequest *aRequest); \
  nsresult OnImageMissing(nsISupports *aUserData); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIALERTNOTIFICATIONIMAGELISTENER(_to) \
  NS_IMETHOD OnImageReady(nsISupports *aUserData, imgIRequest *aRequest) override { return _to OnImageReady(aUserData, aRequest); } \
  NS_IMETHOD OnImageMissing(nsISupports *aUserData) override { return _to OnImageMissing(aUserData); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIALERTNOTIFICATIONIMAGELISTENER(_to) \
  NS_IMETHOD OnImageReady(nsISupports *aUserData, imgIRequest *aRequest) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnImageReady(aUserData, aRequest); } \
  NS_IMETHOD OnImageMissing(nsISupports *aUserData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnImageMissing(aUserData); } 


/* starting interface:    nsIAlertNotification */
#define NS_IALERTNOTIFICATION_IID_STR "cf2e4cb6-4b8f-4eca-aea9-d51a8f9f7a50"

#define NS_IALERTNOTIFICATION_IID \
  {0xcf2e4cb6, 0x4b8f, 0x4eca, \
    { 0xae, 0xa9, 0xd5, 0x1a, 0x8f, 0x9f, 0x7a, 0x50 }}

class NS_NO_VTABLE nsIAlertNotification : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IALERTNOTIFICATION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAlertNotification;

  /* void init ([optional] in AString aName, [optional] in AString aImageURL, [optional] in AString aTitle, [optional] in AString aText, [optional] in boolean aTextClickable, [optional] in AString aCookie, [optional] in AString aDir, [optional] in AString aLang, [optional] in AString aData, [optional] in nsIPrincipal aPrincipal, [optional] in boolean aInPrivateBrowsing, [optional] in boolean aRequireInteraction); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(const nsAString& aName, const nsAString& aImageURL, const nsAString& aTitle, const nsAString& aText, bool aTextClickable, const nsAString& aCookie, const nsAString& aDir, const nsAString& aLang, const nsAString& aData, nsIPrincipal *aPrincipal, bool aInPrivateBrowsing, bool aRequireInteraction) = 0;

  /* readonly attribute AString name; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetName(nsAString& aName) = 0;

  /* readonly attribute AString imageURL; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetImageURL(nsAString& aImageURL) = 0;

  /* readonly attribute AString title; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTitle(nsAString& aTitle) = 0;

  /* readonly attribute AString text; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetText(nsAString& aText) = 0;

  /* readonly attribute boolean textClickable; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTextClickable(bool *aTextClickable) = 0;

  /* readonly attribute AString cookie; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCookie(nsAString& aCookie) = 0;

  /* readonly attribute AString dir; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDir(nsAString& aDir) = 0;

  /* readonly attribute AString lang; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLang(nsAString& aLang) = 0;

  /* readonly attribute AString data; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetData(nsAString& aData) = 0;

  /* readonly attribute nsIPrincipal principal; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) = 0;

  /* readonly attribute nsIURI URI; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetURI(nsIURI **aURI) = 0;

  /* readonly attribute boolean inPrivateBrowsing; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInPrivateBrowsing(bool *aInPrivateBrowsing) = 0;

  /* readonly attribute boolean requireInteraction; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRequireInteraction(bool *aRequireInteraction) = 0;

  /* readonly attribute boolean actionable; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetActionable(bool *aActionable) = 0;

  /* readonly attribute AString source; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSource(nsAString& aSource) = 0;

  /* nsICancelable loadImage (in unsigned long aTimeout, in nsIAlertNotificationImageListener aListener, [optional] in nsISupports aUserData); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD LoadImage(uint32_t aTimeout, nsIAlertNotificationImageListener *aListener, nsISupports *aUserData, nsICancelable **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAlertNotification, NS_IALERTNOTIFICATION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIALERTNOTIFICATION \
  NS_IMETHOD Init(const nsAString& aName, const nsAString& aImageURL, const nsAString& aTitle, const nsAString& aText, bool aTextClickable, const nsAString& aCookie, const nsAString& aDir, const nsAString& aLang, const nsAString& aData, nsIPrincipal *aPrincipal, bool aInPrivateBrowsing, bool aRequireInteraction) override; \
  NS_IMETHOD GetName(nsAString& aName) override; \
  NS_IMETHOD GetImageURL(nsAString& aImageURL) override; \
  NS_IMETHOD GetTitle(nsAString& aTitle) override; \
  NS_IMETHOD GetText(nsAString& aText) override; \
  NS_IMETHOD GetTextClickable(bool *aTextClickable) override; \
  NS_IMETHOD GetCookie(nsAString& aCookie) override; \
  NS_IMETHOD GetDir(nsAString& aDir) override; \
  NS_IMETHOD GetLang(nsAString& aLang) override; \
  NS_IMETHOD GetData(nsAString& aData) override; \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override; \
  NS_IMETHOD GetURI(nsIURI **aURI) override; \
  NS_IMETHOD GetInPrivateBrowsing(bool *aInPrivateBrowsing) override; \
  NS_IMETHOD GetRequireInteraction(bool *aRequireInteraction) override; \
  NS_IMETHOD GetActionable(bool *aActionable) override; \
  NS_IMETHOD GetSource(nsAString& aSource) override; \
  NS_IMETHOD LoadImage(uint32_t aTimeout, nsIAlertNotificationImageListener *aListener, nsISupports *aUserData, nsICancelable **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIALERTNOTIFICATION \
  nsresult Init(const nsAString& aName, const nsAString& aImageURL, const nsAString& aTitle, const nsAString& aText, bool aTextClickable, const nsAString& aCookie, const nsAString& aDir, const nsAString& aLang, const nsAString& aData, nsIPrincipal *aPrincipal, bool aInPrivateBrowsing, bool aRequireInteraction); \
  nsresult GetName(nsAString& aName); \
  nsresult GetImageURL(nsAString& aImageURL); \
  nsresult GetTitle(nsAString& aTitle); \
  nsresult GetText(nsAString& aText); \
  nsresult GetTextClickable(bool *aTextClickable); \
  nsresult GetCookie(nsAString& aCookie); \
  nsresult GetDir(nsAString& aDir); \
  nsresult GetLang(nsAString& aLang); \
  nsresult GetData(nsAString& aData); \
  nsresult GetPrincipal(nsIPrincipal **aPrincipal); \
  nsresult GetURI(nsIURI **aURI); \
  nsresult GetInPrivateBrowsing(bool *aInPrivateBrowsing); \
  nsresult GetRequireInteraction(bool *aRequireInteraction); \
  nsresult GetActionable(bool *aActionable); \
  nsresult GetSource(nsAString& aSource); \
  nsresult LoadImage(uint32_t aTimeout, nsIAlertNotificationImageListener *aListener, nsISupports *aUserData, nsICancelable **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIALERTNOTIFICATION(_to) \
  NS_IMETHOD Init(const nsAString& aName, const nsAString& aImageURL, const nsAString& aTitle, const nsAString& aText, bool aTextClickable, const nsAString& aCookie, const nsAString& aDir, const nsAString& aLang, const nsAString& aData, nsIPrincipal *aPrincipal, bool aInPrivateBrowsing, bool aRequireInteraction) override { return _to Init(aName, aImageURL, aTitle, aText, aTextClickable, aCookie, aDir, aLang, aData, aPrincipal, aInPrivateBrowsing, aRequireInteraction); } \
  NS_IMETHOD GetName(nsAString& aName) override { return _to GetName(aName); } \
  NS_IMETHOD GetImageURL(nsAString& aImageURL) override { return _to GetImageURL(aImageURL); } \
  NS_IMETHOD GetTitle(nsAString& aTitle) override { return _to GetTitle(aTitle); } \
  NS_IMETHOD GetText(nsAString& aText) override { return _to GetText(aText); } \
  NS_IMETHOD GetTextClickable(bool *aTextClickable) override { return _to GetTextClickable(aTextClickable); } \
  NS_IMETHOD GetCookie(nsAString& aCookie) override { return _to GetCookie(aCookie); } \
  NS_IMETHOD GetDir(nsAString& aDir) override { return _to GetDir(aDir); } \
  NS_IMETHOD GetLang(nsAString& aLang) override { return _to GetLang(aLang); } \
  NS_IMETHOD GetData(nsAString& aData) override { return _to GetData(aData); } \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override { return _to GetPrincipal(aPrincipal); } \
  NS_IMETHOD GetURI(nsIURI **aURI) override { return _to GetURI(aURI); } \
  NS_IMETHOD GetInPrivateBrowsing(bool *aInPrivateBrowsing) override { return _to GetInPrivateBrowsing(aInPrivateBrowsing); } \
  NS_IMETHOD GetRequireInteraction(bool *aRequireInteraction) override { return _to GetRequireInteraction(aRequireInteraction); } \
  NS_IMETHOD GetActionable(bool *aActionable) override { return _to GetActionable(aActionable); } \
  NS_IMETHOD GetSource(nsAString& aSource) override { return _to GetSource(aSource); } \
  NS_IMETHOD LoadImage(uint32_t aTimeout, nsIAlertNotificationImageListener *aListener, nsISupports *aUserData, nsICancelable **_retval) override { return _to LoadImage(aTimeout, aListener, aUserData, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIALERTNOTIFICATION(_to) \
  NS_IMETHOD Init(const nsAString& aName, const nsAString& aImageURL, const nsAString& aTitle, const nsAString& aText, bool aTextClickable, const nsAString& aCookie, const nsAString& aDir, const nsAString& aLang, const nsAString& aData, nsIPrincipal *aPrincipal, bool aInPrivateBrowsing, bool aRequireInteraction) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aName, aImageURL, aTitle, aText, aTextClickable, aCookie, aDir, aLang, aData, aPrincipal, aInPrivateBrowsing, aRequireInteraction); } \
  NS_IMETHOD GetName(nsAString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetName(aName); } \
  NS_IMETHOD GetImageURL(nsAString& aImageURL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetImageURL(aImageURL); } \
  NS_IMETHOD GetTitle(nsAString& aTitle) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTitle(aTitle); } \
  NS_IMETHOD GetText(nsAString& aText) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetText(aText); } \
  NS_IMETHOD GetTextClickable(bool *aTextClickable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTextClickable(aTextClickable); } \
  NS_IMETHOD GetCookie(nsAString& aCookie) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCookie(aCookie); } \
  NS_IMETHOD GetDir(nsAString& aDir) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDir(aDir); } \
  NS_IMETHOD GetLang(nsAString& aLang) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLang(aLang); } \
  NS_IMETHOD GetData(nsAString& aData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetData(aData); } \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrincipal(aPrincipal); } \
  NS_IMETHOD GetURI(nsIURI **aURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetURI(aURI); } \
  NS_IMETHOD GetInPrivateBrowsing(bool *aInPrivateBrowsing) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInPrivateBrowsing(aInPrivateBrowsing); } \
  NS_IMETHOD GetRequireInteraction(bool *aRequireInteraction) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRequireInteraction(aRequireInteraction); } \
  NS_IMETHOD GetActionable(bool *aActionable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetActionable(aActionable); } \
  NS_IMETHOD GetSource(nsAString& aSource) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSource(aSource); } \
  NS_IMETHOD LoadImage(uint32_t aTimeout, nsIAlertNotificationImageListener *aListener, nsISupports *aUserData, nsICancelable **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LoadImage(aTimeout, aListener, aUserData, _retval); } 


/* starting interface:    nsIAlertsService */
#define NS_IALERTSSERVICE_IID_STR "f7a36392-d98b-4141-a7d7-4e46642684e3"

#define NS_IALERTSSERVICE_IID \
  {0xf7a36392, 0xd98b, 0x4141, \
    { 0xa7, 0xd7, 0x4e, 0x46, 0x64, 0x26, 0x84, 0xe3 }}

class NS_NO_VTABLE nsIAlertsService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IALERTSSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAlertsService;

  /* void showPersistentNotification (in AString aPersistentData, in nsIAlertNotification aAlert, [optional] in nsIObserver aAlertListener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ShowPersistentNotification(const nsAString& aPersistentData, nsIAlertNotification *aAlert, nsIObserver *aAlertListener) = 0;

  /* void showAlert (in nsIAlertNotification aAlert, [optional] in nsIObserver aAlertListener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ShowAlert(nsIAlertNotification *aAlert, nsIObserver *aAlertListener) = 0;

  /* void showAlertNotification (in AString aImageURL, in AString aTitle, in AString aText, [optional] in boolean aTextClickable, [optional] in AString aCookie, [optional] in nsIObserver aAlertListener, [optional] in AString aName, [optional] in AString aDir, [optional] in AString aLang, [optional] in AString aData, [optional] in nsIPrincipal aPrincipal, [optional] in boolean aInPrivateBrowsing, [optional] in boolean aRequireInteraction); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ShowAlertNotification(const nsAString& aImageURL, const nsAString& aTitle, const nsAString& aText, bool aTextClickable, const nsAString& aCookie, nsIObserver *aAlertListener, const nsAString& aName, const nsAString& aDir, const nsAString& aLang, const nsAString& aData, nsIPrincipal *aPrincipal, bool aInPrivateBrowsing, bool aRequireInteraction) = 0;

  /* void closeAlert ([optional] in AString aName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CloseAlert(const nsAString& aName) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAlertsService, NS_IALERTSSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIALERTSSERVICE \
  NS_IMETHOD ShowPersistentNotification(const nsAString& aPersistentData, nsIAlertNotification *aAlert, nsIObserver *aAlertListener) override; \
  NS_IMETHOD ShowAlert(nsIAlertNotification *aAlert, nsIObserver *aAlertListener) override; \
  NS_IMETHOD ShowAlertNotification(const nsAString& aImageURL, const nsAString& aTitle, const nsAString& aText, bool aTextClickable, const nsAString& aCookie, nsIObserver *aAlertListener, const nsAString& aName, const nsAString& aDir, const nsAString& aLang, const nsAString& aData, nsIPrincipal *aPrincipal, bool aInPrivateBrowsing, bool aRequireInteraction) override; \
  NS_IMETHOD CloseAlert(const nsAString& aName) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIALERTSSERVICE \
  nsresult ShowPersistentNotification(const nsAString& aPersistentData, nsIAlertNotification *aAlert, nsIObserver *aAlertListener); \
  nsresult ShowAlert(nsIAlertNotification *aAlert, nsIObserver *aAlertListener); \
  nsresult ShowAlertNotification(const nsAString& aImageURL, const nsAString& aTitle, const nsAString& aText, bool aTextClickable, const nsAString& aCookie, nsIObserver *aAlertListener, const nsAString& aName, const nsAString& aDir, const nsAString& aLang, const nsAString& aData, nsIPrincipal *aPrincipal, bool aInPrivateBrowsing, bool aRequireInteraction); \
  nsresult CloseAlert(const nsAString& aName); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIALERTSSERVICE(_to) \
  NS_IMETHOD ShowPersistentNotification(const nsAString& aPersistentData, nsIAlertNotification *aAlert, nsIObserver *aAlertListener) override { return _to ShowPersistentNotification(aPersistentData, aAlert, aAlertListener); } \
  NS_IMETHOD ShowAlert(nsIAlertNotification *aAlert, nsIObserver *aAlertListener) override { return _to ShowAlert(aAlert, aAlertListener); } \
  NS_IMETHOD ShowAlertNotification(const nsAString& aImageURL, const nsAString& aTitle, const nsAString& aText, bool aTextClickable, const nsAString& aCookie, nsIObserver *aAlertListener, const nsAString& aName, const nsAString& aDir, const nsAString& aLang, const nsAString& aData, nsIPrincipal *aPrincipal, bool aInPrivateBrowsing, bool aRequireInteraction) override { return _to ShowAlertNotification(aImageURL, aTitle, aText, aTextClickable, aCookie, aAlertListener, aName, aDir, aLang, aData, aPrincipal, aInPrivateBrowsing, aRequireInteraction); } \
  NS_IMETHOD CloseAlert(const nsAString& aName) override { return _to CloseAlert(aName); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIALERTSSERVICE(_to) \
  NS_IMETHOD ShowPersistentNotification(const nsAString& aPersistentData, nsIAlertNotification *aAlert, nsIObserver *aAlertListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ShowPersistentNotification(aPersistentData, aAlert, aAlertListener); } \
  NS_IMETHOD ShowAlert(nsIAlertNotification *aAlert, nsIObserver *aAlertListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ShowAlert(aAlert, aAlertListener); } \
  NS_IMETHOD ShowAlertNotification(const nsAString& aImageURL, const nsAString& aTitle, const nsAString& aText, bool aTextClickable, const nsAString& aCookie, nsIObserver *aAlertListener, const nsAString& aName, const nsAString& aDir, const nsAString& aLang, const nsAString& aData, nsIPrincipal *aPrincipal, bool aInPrivateBrowsing, bool aRequireInteraction) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ShowAlertNotification(aImageURL, aTitle, aText, aTextClickable, aCookie, aAlertListener, aName, aDir, aLang, aData, aPrincipal, aInPrivateBrowsing, aRequireInteraction); } \
  NS_IMETHOD CloseAlert(const nsAString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CloseAlert(aName); } 


/* starting interface:    nsIAlertsDoNotDisturb */
#define NS_IALERTSDONOTDISTURB_IID_STR "c5d63e3a-259d-45a8-b964-8377967cb4d2"

#define NS_IALERTSDONOTDISTURB_IID \
  {0xc5d63e3a, 0x259d, 0x45a8, \
    { 0xb9, 0x64, 0x83, 0x77, 0x96, 0x7c, 0xb4, 0xd2 }}

class NS_NO_VTABLE nsIAlertsDoNotDisturb : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IALERTSDONOTDISTURB_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAlertsDoNotDisturb;

  /* attribute bool manualDoNotDisturb; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetManualDoNotDisturb(bool *aManualDoNotDisturb) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetManualDoNotDisturb(bool aManualDoNotDisturb) = 0;

  /* attribute bool suppressForScreenSharing; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSuppressForScreenSharing(bool *aSuppressForScreenSharing) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSuppressForScreenSharing(bool aSuppressForScreenSharing) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAlertsDoNotDisturb, NS_IALERTSDONOTDISTURB_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIALERTSDONOTDISTURB \
  NS_IMETHOD GetManualDoNotDisturb(bool *aManualDoNotDisturb) override; \
  NS_IMETHOD SetManualDoNotDisturb(bool aManualDoNotDisturb) override; \
  NS_IMETHOD GetSuppressForScreenSharing(bool *aSuppressForScreenSharing) override; \
  NS_IMETHOD SetSuppressForScreenSharing(bool aSuppressForScreenSharing) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIALERTSDONOTDISTURB \
  nsresult GetManualDoNotDisturb(bool *aManualDoNotDisturb); \
  nsresult SetManualDoNotDisturb(bool aManualDoNotDisturb); \
  nsresult GetSuppressForScreenSharing(bool *aSuppressForScreenSharing); \
  nsresult SetSuppressForScreenSharing(bool aSuppressForScreenSharing); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIALERTSDONOTDISTURB(_to) \
  NS_IMETHOD GetManualDoNotDisturb(bool *aManualDoNotDisturb) override { return _to GetManualDoNotDisturb(aManualDoNotDisturb); } \
  NS_IMETHOD SetManualDoNotDisturb(bool aManualDoNotDisturb) override { return _to SetManualDoNotDisturb(aManualDoNotDisturb); } \
  NS_IMETHOD GetSuppressForScreenSharing(bool *aSuppressForScreenSharing) override { return _to GetSuppressForScreenSharing(aSuppressForScreenSharing); } \
  NS_IMETHOD SetSuppressForScreenSharing(bool aSuppressForScreenSharing) override { return _to SetSuppressForScreenSharing(aSuppressForScreenSharing); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIALERTSDONOTDISTURB(_to) \
  NS_IMETHOD GetManualDoNotDisturb(bool *aManualDoNotDisturb) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetManualDoNotDisturb(aManualDoNotDisturb); } \
  NS_IMETHOD SetManualDoNotDisturb(bool aManualDoNotDisturb) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetManualDoNotDisturb(aManualDoNotDisturb); } \
  NS_IMETHOD GetSuppressForScreenSharing(bool *aSuppressForScreenSharing) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSuppressForScreenSharing(aSuppressForScreenSharing); } \
  NS_IMETHOD SetSuppressForScreenSharing(bool aSuppressForScreenSharing) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSuppressForScreenSharing(aSuppressForScreenSharing); } 


/* starting interface:    nsIAlertsIconData */
#define NS_IALERTSICONDATA_IID_STR "fc6d7f0a-0cf6-4268-8c71-ab640842b9b1"

#define NS_IALERTSICONDATA_IID \
  {0xfc6d7f0a, 0x0cf6, 0x4268, \
    { 0x8c, 0x71, 0xab, 0x64, 0x08, 0x42, 0xb9, 0xb1 }}

class NS_NO_VTABLE nsIAlertsIconData : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IALERTSICONDATA_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAlertsIconData;

  /* void showAlertWithIconData (in nsIAlertNotification aAlert, [optional] in nsIObserver aAlertListener, [optional] in uint32_t aIconSize, [array, size_is (aIconSize), const] in uint8_t aIconData); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ShowAlertWithIconData(nsIAlertNotification *aAlert, nsIObserver *aAlertListener, uint32_t aIconSize, const uint8_t *aIconData) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAlertsIconData, NS_IALERTSICONDATA_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIALERTSICONDATA \
  NS_IMETHOD ShowAlertWithIconData(nsIAlertNotification *aAlert, nsIObserver *aAlertListener, uint32_t aIconSize, const uint8_t *aIconData) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIALERTSICONDATA \
  nsresult ShowAlertWithIconData(nsIAlertNotification *aAlert, nsIObserver *aAlertListener, uint32_t aIconSize, const uint8_t *aIconData); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIALERTSICONDATA(_to) \
  NS_IMETHOD ShowAlertWithIconData(nsIAlertNotification *aAlert, nsIObserver *aAlertListener, uint32_t aIconSize, const uint8_t *aIconData) override { return _to ShowAlertWithIconData(aAlert, aAlertListener, aIconSize, aIconData); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIALERTSICONDATA(_to) \
  NS_IMETHOD ShowAlertWithIconData(nsIAlertNotification *aAlert, nsIObserver *aAlertListener, uint32_t aIconSize, const uint8_t *aIconData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ShowAlertWithIconData(aAlert, aAlertListener, aIconSize, aIconData); } 


/* starting interface:    nsIAlertsIconURI */
#define NS_IALERTSICONURI_IID_STR "f3c82915-bf60-41ea-91ce-6c46b22e381a"

#define NS_IALERTSICONURI_IID \
  {0xf3c82915, 0xbf60, 0x41ea, \
    { 0x91, 0xce, 0x6c, 0x46, 0xb2, 0x2e, 0x38, 0x1a }}

class NS_NO_VTABLE nsIAlertsIconURI : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IALERTSICONURI_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAlertsIconURI;

  /* void showAlertWithIconURI (in nsIAlertNotification aAlert, [optional] in nsIObserver aAlertListener, [optional] in nsIURI aIconURI); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ShowAlertWithIconURI(nsIAlertNotification *aAlert, nsIObserver *aAlertListener, nsIURI *aIconURI) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAlertsIconURI, NS_IALERTSICONURI_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIALERTSICONURI \
  NS_IMETHOD ShowAlertWithIconURI(nsIAlertNotification *aAlert, nsIObserver *aAlertListener, nsIURI *aIconURI) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIALERTSICONURI \
  nsresult ShowAlertWithIconURI(nsIAlertNotification *aAlert, nsIObserver *aAlertListener, nsIURI *aIconURI); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIALERTSICONURI(_to) \
  NS_IMETHOD ShowAlertWithIconURI(nsIAlertNotification *aAlert, nsIObserver *aAlertListener, nsIURI *aIconURI) override { return _to ShowAlertWithIconURI(aAlert, aAlertListener, aIconURI); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIALERTSICONURI(_to) \
  NS_IMETHOD ShowAlertWithIconURI(nsIAlertNotification *aAlert, nsIObserver *aAlertListener, nsIURI *aIconURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ShowAlertWithIconURI(aAlert, aAlertListener, aIconURI); } 


#endif /* __gen_nsIAlertsService_h__ */
