/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/parentalcontrols/nsIParentalControlsService.idl
 */

#ifndef __gen_nsIParentalControlsService_h__
#define __gen_nsIParentalControlsService_h__


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

class nsIInterfaceRequestor; /* forward declaration */

class nsIArray; /* forward declaration */


/* starting interface:    nsIParentalControlsService */
#define NS_IPARENTALCONTROLSSERVICE_IID_STR "2e97e5dd-467b-4aea-a1bb-6773c0f2beb0"

#define NS_IPARENTALCONTROLSSERVICE_IID \
  {0x2e97e5dd, 0x467b, 0x4aea, \
    { 0xa1, 0xbb, 0x67, 0x73, 0xc0, 0xf2, 0xbe, 0xb0 }}

class NS_NO_VTABLE nsIParentalControlsService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPARENTALCONTROLSSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIParentalControlsService;

  enum {
    DOWNLOAD = 1,
    INSTALL_EXTENSION = 2,
    INSTALL_APP = 3,
    BROWSE = 4,
    SHARE = 5,
    BOOKMARK = 6,
    ADD_CONTACT = 7,
    SET_IMAGE = 8,
    MODIFY_ACCOUNTS = 9,
    REMOTE_DEBUGGING = 10,
    IMPORT_SETTINGS = 11,
    PRIVATE_BROWSING = 12,
    DATA_CHOICES = 13,
    CLEAR_HISTORY = 14,
    MASTER_PASSWORD = 15,
    GUEST_BROWSING = 16,
    ADVANCED_SETTINGS = 17,
    CAMERA_MICROPHONE = 18,
    BLOCK_LIST = 19,
    TELEMETRY = 20,
    HEALTH_REPORT = 21,
    DEFAULT_THEME = 22
  };

  /* readonly attribute boolean parentalControlsEnabled; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetParentalControlsEnabled(bool *aParentalControlsEnabled) = 0;

  /* readonly attribute boolean blockFileDownloadsEnabled; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBlockFileDownloadsEnabled(bool *aBlockFileDownloadsEnabled) = 0;

  /* boolean isAllowed (in short aAction, [optional] in nsIURI aUri); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsAllowed(int16_t aAction, nsIURI *aUri, bool *_retval) = 0;

  /* readonly attribute boolean loggingEnabled; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLoggingEnabled(bool *aLoggingEnabled) = 0;

  enum {
    ePCLog_URIVisit = 1,
    ePCLog_FileDownload = 2
  };

  /* void log (in short aEntryType, in boolean aFlag, in nsIURI aSource, [optional] in nsIFile aTarget); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Log(int16_t aEntryType, bool aFlag, nsIURI *aSource, nsIFile *aTarget) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIParentalControlsService, NS_IPARENTALCONTROLSSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPARENTALCONTROLSSERVICE \
  NS_IMETHOD GetParentalControlsEnabled(bool *aParentalControlsEnabled) override; \
  NS_IMETHOD GetBlockFileDownloadsEnabled(bool *aBlockFileDownloadsEnabled) override; \
  NS_IMETHOD IsAllowed(int16_t aAction, nsIURI *aUri, bool *_retval) override; \
  NS_IMETHOD GetLoggingEnabled(bool *aLoggingEnabled) override; \
  NS_IMETHOD Log(int16_t aEntryType, bool aFlag, nsIURI *aSource, nsIFile *aTarget) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPARENTALCONTROLSSERVICE \
  nsresult GetParentalControlsEnabled(bool *aParentalControlsEnabled); \
  nsresult GetBlockFileDownloadsEnabled(bool *aBlockFileDownloadsEnabled); \
  nsresult IsAllowed(int16_t aAction, nsIURI *aUri, bool *_retval); \
  nsresult GetLoggingEnabled(bool *aLoggingEnabled); \
  nsresult Log(int16_t aEntryType, bool aFlag, nsIURI *aSource, nsIFile *aTarget); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPARENTALCONTROLSSERVICE(_to) \
  NS_IMETHOD GetParentalControlsEnabled(bool *aParentalControlsEnabled) override { return _to GetParentalControlsEnabled(aParentalControlsEnabled); } \
  NS_IMETHOD GetBlockFileDownloadsEnabled(bool *aBlockFileDownloadsEnabled) override { return _to GetBlockFileDownloadsEnabled(aBlockFileDownloadsEnabled); } \
  NS_IMETHOD IsAllowed(int16_t aAction, nsIURI *aUri, bool *_retval) override { return _to IsAllowed(aAction, aUri, _retval); } \
  NS_IMETHOD GetLoggingEnabled(bool *aLoggingEnabled) override { return _to GetLoggingEnabled(aLoggingEnabled); } \
  NS_IMETHOD Log(int16_t aEntryType, bool aFlag, nsIURI *aSource, nsIFile *aTarget) override { return _to Log(aEntryType, aFlag, aSource, aTarget); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPARENTALCONTROLSSERVICE(_to) \
  NS_IMETHOD GetParentalControlsEnabled(bool *aParentalControlsEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetParentalControlsEnabled(aParentalControlsEnabled); } \
  NS_IMETHOD GetBlockFileDownloadsEnabled(bool *aBlockFileDownloadsEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBlockFileDownloadsEnabled(aBlockFileDownloadsEnabled); } \
  NS_IMETHOD IsAllowed(int16_t aAction, nsIURI *aUri, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsAllowed(aAction, aUri, _retval); } \
  NS_IMETHOD GetLoggingEnabled(bool *aLoggingEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoggingEnabled(aLoggingEnabled); } \
  NS_IMETHOD Log(int16_t aEntryType, bool aFlag, nsIURI *aSource, nsIFile *aTarget) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Log(aEntryType, aFlag, aSource, aTarget); } 


#endif /* __gen_nsIParentalControlsService_h__ */
