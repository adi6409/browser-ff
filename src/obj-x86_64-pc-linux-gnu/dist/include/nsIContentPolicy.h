/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsIContentPolicy.idl
 */

#ifndef __gen_nsIContentPolicy_h__
#define __gen_nsIContentPolicy_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsILoadInfo; /* forward declaration */


/* starting interface:    nsIContentPolicy */
#define NS_ICONTENTPOLICY_IID_STR "caad4f1f-d047-46ac-ae9d-dc598e4fb91b"

#define NS_ICONTENTPOLICY_IID \
  {0xcaad4f1f, 0xd047, 0x46ac, \
    { 0xae, 0x9d, 0xdc, 0x59, 0x8e, 0x4f, 0xb9, 0x1b }}

class NS_NO_VTABLE nsIContentPolicy : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICONTENTPOLICY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIContentPolicy;

  enum nsContentPolicyType : uint8_t {
    TYPE_INVALID = 0,
    TYPE_OTHER = 1,
    TYPE_SCRIPT = 2,
    TYPE_IMAGE = 3,
    TYPE_STYLESHEET = 4,
    TYPE_OBJECT = 5,
    TYPE_DOCUMENT = 6,
    TYPE_SUBDOCUMENT = 7,
    TYPE_PING = 10,
    TYPE_XMLHTTPREQUEST = 11,
    TYPE_OBJECT_SUBREQUEST = 12,
    TYPE_DTD = 13,
    TYPE_FONT = 14,
    TYPE_MEDIA = 15,
    TYPE_WEBSOCKET = 16,
    TYPE_CSP_REPORT = 17,
    TYPE_XSLT = 18,
    TYPE_BEACON = 19,
    TYPE_FETCH = 20,
    TYPE_IMAGESET = 21,
    TYPE_WEB_MANIFEST = 22,
    TYPE_INTERNAL_SCRIPT = 23,
    TYPE_INTERNAL_WORKER = 24,
    TYPE_INTERNAL_SHARED_WORKER = 25,
    TYPE_INTERNAL_EMBED = 26,
    TYPE_INTERNAL_OBJECT = 27,
    TYPE_INTERNAL_FRAME = 28,
    TYPE_INTERNAL_IFRAME = 29,
    TYPE_INTERNAL_AUDIO = 30,
    TYPE_INTERNAL_VIDEO = 31,
    TYPE_INTERNAL_TRACK = 32,
    TYPE_INTERNAL_XMLHTTPREQUEST = 33,
    TYPE_INTERNAL_EVENTSOURCE = 34,
    TYPE_INTERNAL_SERVICE_WORKER = 35,
    TYPE_INTERNAL_SCRIPT_PRELOAD = 36,
    TYPE_INTERNAL_IMAGE = 37,
    TYPE_INTERNAL_IMAGE_PRELOAD = 38,
    TYPE_INTERNAL_STYLESHEET = 39,
    TYPE_INTERNAL_STYLESHEET_PRELOAD = 40,
    TYPE_INTERNAL_IMAGE_FAVICON = 41,
    TYPE_INTERNAL_WORKER_IMPORT_SCRIPTS = 42,
    TYPE_SAVEAS_DOWNLOAD = 43,
    TYPE_SPECULATIVE = 44,
    TYPE_INTERNAL_MODULE = 45,
    TYPE_INTERNAL_MODULE_PRELOAD = 46,
    TYPE_INTERNAL_DTD = 47,
    TYPE_INTERNAL_FORCE_ALLOWED_DTD = 48,
    TYPE_INTERNAL_AUDIOWORKLET = 49,
    TYPE_INTERNAL_PAINTWORKLET = 50,
    TYPE_INTERNAL_FONT_PRELOAD = 51,
    TYPE_INTERNAL_CHROMEUTILS_COMPILED_SCRIPT = 52,
    TYPE_INTERNAL_FRAME_MESSAGEMANAGER_SCRIPT = 53,
    TYPE_INTERNAL_FETCH_PRELOAD = 54,
  };

  enum {
    REJECT_REQUEST = -1,
    REJECT_TYPE = -2,
    REJECT_SERVER = -3,
    REJECT_OTHER = -4,
    REJECT_POLICY = -5,
    ACCEPT = 1
  };

  /* short shouldLoad (in nsIURI aContentLocation, in nsILoadInfo aLoadInfo, in ACString aMimeTypeGuess); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ShouldLoad(nsIURI *aContentLocation, nsILoadInfo *aLoadInfo, const nsACString& aMimeTypeGuess, int16_t *_retval) = 0;

  /* short shouldProcess (in nsIURI aContentLocation, in nsILoadInfo aLoadInfo, in ACString aMimeType); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ShouldProcess(nsIURI *aContentLocation, nsILoadInfo *aLoadInfo, const nsACString& aMimeType, int16_t *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIContentPolicy, NS_ICONTENTPOLICY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICONTENTPOLICY \
  NS_IMETHOD ShouldLoad(nsIURI *aContentLocation, nsILoadInfo *aLoadInfo, const nsACString& aMimeTypeGuess, int16_t *_retval) override; \
  NS_IMETHOD ShouldProcess(nsIURI *aContentLocation, nsILoadInfo *aLoadInfo, const nsACString& aMimeType, int16_t *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICONTENTPOLICY \
  nsresult ShouldLoad(nsIURI *aContentLocation, nsILoadInfo *aLoadInfo, const nsACString& aMimeTypeGuess, int16_t *_retval); \
  nsresult ShouldProcess(nsIURI *aContentLocation, nsILoadInfo *aLoadInfo, const nsACString& aMimeType, int16_t *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICONTENTPOLICY(_to) \
  NS_IMETHOD ShouldLoad(nsIURI *aContentLocation, nsILoadInfo *aLoadInfo, const nsACString& aMimeTypeGuess, int16_t *_retval) override { return _to ShouldLoad(aContentLocation, aLoadInfo, aMimeTypeGuess, _retval); } \
  NS_IMETHOD ShouldProcess(nsIURI *aContentLocation, nsILoadInfo *aLoadInfo, const nsACString& aMimeType, int16_t *_retval) override { return _to ShouldProcess(aContentLocation, aLoadInfo, aMimeType, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICONTENTPOLICY(_to) \
  NS_IMETHOD ShouldLoad(nsIURI *aContentLocation, nsILoadInfo *aLoadInfo, const nsACString& aMimeTypeGuess, int16_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ShouldLoad(aContentLocation, aLoadInfo, aMimeTypeGuess, _retval); } \
  NS_IMETHOD ShouldProcess(nsIURI *aContentLocation, nsILoadInfo *aLoadInfo, const nsACString& aMimeType, int16_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ShouldProcess(aContentLocation, aLoadInfo, aMimeType, _retval); } 

typedef nsIContentPolicy::nsContentPolicyType  nsContentPolicyType;

enum class ExtContentPolicyType : uint8_t {
  /**
   * The type of ExtContentPolicy::TYPE_*
   */
  TYPE_INVALID = nsIContentPolicy::TYPE_INVALID,
  TYPE_OTHER = nsIContentPolicy::TYPE_OTHER,
  TYPE_SCRIPT = nsIContentPolicy::TYPE_SCRIPT,
  TYPE_IMAGE = nsIContentPolicy::TYPE_IMAGE,
  TYPE_STYLESHEET = nsIContentPolicy::TYPE_STYLESHEET,
  TYPE_OBJECT = nsIContentPolicy::TYPE_OBJECT,
  TYPE_DOCUMENT = nsIContentPolicy::TYPE_DOCUMENT,
  TYPE_SUBDOCUMENT = nsIContentPolicy::TYPE_SUBDOCUMENT,
  TYPE_PING = nsIContentPolicy::TYPE_PING,
  TYPE_XMLHTTPREQUEST = nsIContentPolicy::TYPE_XMLHTTPREQUEST,
  TYPE_OBJECT_SUBREQUEST = nsIContentPolicy::TYPE_OBJECT_SUBREQUEST,
  TYPE_DTD = nsIContentPolicy::TYPE_DTD,
  TYPE_FONT = nsIContentPolicy::TYPE_FONT,
  TYPE_MEDIA = nsIContentPolicy::TYPE_MEDIA,
  TYPE_WEBSOCKET = nsIContentPolicy::TYPE_WEBSOCKET,
  TYPE_CSP_REPORT = nsIContentPolicy::TYPE_CSP_REPORT,
  TYPE_XSLT = nsIContentPolicy::TYPE_XSLT,
  TYPE_BEACON = nsIContentPolicy::TYPE_BEACON,
  TYPE_FETCH = nsIContentPolicy::TYPE_FETCH,
  TYPE_IMAGESET = nsIContentPolicy::TYPE_IMAGESET,
  TYPE_WEB_MANIFEST = nsIContentPolicy::TYPE_WEB_MANIFEST,
  TYPE_SAVEAS_DOWNLOAD = nsIContentPolicy::TYPE_SAVEAS_DOWNLOAD,
  TYPE_SPECULATIVE = nsIContentPolicy::TYPE_SPECULATIVE,
};
typedef ExtContentPolicyType ExtContentPolicy;

#endif /* __gen_nsIContentPolicy_h__ */
