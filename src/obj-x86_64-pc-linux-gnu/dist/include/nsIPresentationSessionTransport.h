/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationSessionTransport.idl
 */

#ifndef __gen_nsIPresentationSessionTransport_h__
#define __gen_nsIPresentationSessionTransport_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIInputStream; /* forward declaration */

class nsINetAddr; /* forward declaration */

namespace mozilla {
namespace dom {
class Blob; /* webidl Blob */
} // namespace dom
} // namespace mozilla

#define PRESENTATION_TCP_SESSION_TRANSPORT_CONTRACTID \
  "@mozilla.org/presentation/presentationtcpsessiontransport;1"

/* starting interface:    nsIPresentationSessionTransportCallback */
#define NS_IPRESENTATIONSESSIONTRANSPORTCALLBACK_IID_STR "9f158786-41a6-4a10-b29b-9497f25d4b67"

#define NS_IPRESENTATIONSESSIONTRANSPORTCALLBACK_IID \
  {0x9f158786, 0x41a6, 0x4a10, \
    { 0xb2, 0x9b, 0x94, 0x97, 0xf2, 0x5d, 0x4b, 0x67 }}

class NS_NO_VTABLE nsIPresentationSessionTransportCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRESENTATIONSESSIONTRANSPORTCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPresentationSessionTransportCallback;

  /* void notifyTransportReady (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NotifyTransportReady(void) = 0;

  /* void notifyTransportClosed (in nsresult reason); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NotifyTransportClosed(nsresult reason) = 0;

  /* void notifyData (in ACString data, in boolean isBinary); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NotifyData(const nsACString& data, bool isBinary) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPresentationSessionTransportCallback, NS_IPRESENTATIONSESSIONTRANSPORTCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRESENTATIONSESSIONTRANSPORTCALLBACK \
  NS_IMETHOD NotifyTransportReady(void) override; \
  NS_IMETHOD NotifyTransportClosed(nsresult reason) override; \
  NS_IMETHOD NotifyData(const nsACString& data, bool isBinary) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRESENTATIONSESSIONTRANSPORTCALLBACK \
  nsresult NotifyTransportReady(void); \
  nsresult NotifyTransportClosed(nsresult reason); \
  nsresult NotifyData(const nsACString& data, bool isBinary); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRESENTATIONSESSIONTRANSPORTCALLBACK(_to) \
  NS_IMETHOD NotifyTransportReady(void) override { return _to NotifyTransportReady(); } \
  NS_IMETHOD NotifyTransportClosed(nsresult reason) override { return _to NotifyTransportClosed(reason); } \
  NS_IMETHOD NotifyData(const nsACString& data, bool isBinary) override { return _to NotifyData(data, isBinary); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRESENTATIONSESSIONTRANSPORTCALLBACK(_to) \
  NS_IMETHOD NotifyTransportReady(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyTransportReady(); } \
  NS_IMETHOD NotifyTransportClosed(nsresult reason) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyTransportClosed(reason); } \
  NS_IMETHOD NotifyData(const nsACString& data, bool isBinary) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyData(data, isBinary); } 


/* starting interface:    nsIPresentationSessionTransport */
#define NS_IPRESENTATIONSESSIONTRANSPORT_IID_STR "670b7e1b-65be-42b6-a596-be571907fa18"

#define NS_IPRESENTATIONSESSIONTRANSPORT_IID \
  {0x670b7e1b, 0x65be, 0x42b6, \
    { 0xa5, 0x96, 0xbe, 0x57, 0x19, 0x07, 0xfa, 0x18 }}

class NS_NO_VTABLE nsIPresentationSessionTransport : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRESENTATIONSESSIONTRANSPORT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPresentationSessionTransport;

  /* attribute nsIPresentationSessionTransportCallback callback; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCallback(nsIPresentationSessionTransportCallback **aCallback) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCallback(nsIPresentationSessionTransportCallback *aCallback) = 0;

  /* readonly attribute nsINetAddr selfAddress; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSelfAddress(nsINetAddr **aSelfAddress) = 0;

  /* void enableDataNotification (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EnableDataNotification(void) = 0;

  /* void send (in AString data); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Send(const nsAString& data) = 0;

  /* void sendBinaryMsg (in ACString data); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SendBinaryMsg(const nsACString& data) = 0;

  /* void sendBlob (in Blob blob); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SendBlob(mozilla::dom::Blob *blob) = 0;

  /* void close (in nsresult reason); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Close(nsresult reason) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPresentationSessionTransport, NS_IPRESENTATIONSESSIONTRANSPORT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRESENTATIONSESSIONTRANSPORT \
  NS_IMETHOD GetCallback(nsIPresentationSessionTransportCallback **aCallback) override; \
  NS_IMETHOD SetCallback(nsIPresentationSessionTransportCallback *aCallback) override; \
  NS_IMETHOD GetSelfAddress(nsINetAddr **aSelfAddress) override; \
  NS_IMETHOD EnableDataNotification(void) override; \
  NS_IMETHOD Send(const nsAString& data) override; \
  NS_IMETHOD SendBinaryMsg(const nsACString& data) override; \
  NS_IMETHOD SendBlob(mozilla::dom::Blob *blob) override; \
  NS_IMETHOD Close(nsresult reason) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRESENTATIONSESSIONTRANSPORT \
  nsresult GetCallback(nsIPresentationSessionTransportCallback **aCallback); \
  nsresult SetCallback(nsIPresentationSessionTransportCallback *aCallback); \
  nsresult GetSelfAddress(nsINetAddr **aSelfAddress); \
  nsresult EnableDataNotification(void); \
  nsresult Send(const nsAString& data); \
  nsresult SendBinaryMsg(const nsACString& data); \
  nsresult SendBlob(mozilla::dom::Blob *blob); \
  nsresult Close(nsresult reason); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRESENTATIONSESSIONTRANSPORT(_to) \
  NS_IMETHOD GetCallback(nsIPresentationSessionTransportCallback **aCallback) override { return _to GetCallback(aCallback); } \
  NS_IMETHOD SetCallback(nsIPresentationSessionTransportCallback *aCallback) override { return _to SetCallback(aCallback); } \
  NS_IMETHOD GetSelfAddress(nsINetAddr **aSelfAddress) override { return _to GetSelfAddress(aSelfAddress); } \
  NS_IMETHOD EnableDataNotification(void) override { return _to EnableDataNotification(); } \
  NS_IMETHOD Send(const nsAString& data) override { return _to Send(data); } \
  NS_IMETHOD SendBinaryMsg(const nsACString& data) override { return _to SendBinaryMsg(data); } \
  NS_IMETHOD SendBlob(mozilla::dom::Blob *blob) override { return _to SendBlob(blob); } \
  NS_IMETHOD Close(nsresult reason) override { return _to Close(reason); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRESENTATIONSESSIONTRANSPORT(_to) \
  NS_IMETHOD GetCallback(nsIPresentationSessionTransportCallback **aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCallback(aCallback); } \
  NS_IMETHOD SetCallback(nsIPresentationSessionTransportCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCallback(aCallback); } \
  NS_IMETHOD GetSelfAddress(nsINetAddr **aSelfAddress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelfAddress(aSelfAddress); } \
  NS_IMETHOD EnableDataNotification(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnableDataNotification(); } \
  NS_IMETHOD Send(const nsAString& data) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Send(data); } \
  NS_IMETHOD SendBinaryMsg(const nsACString& data) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendBinaryMsg(data); } \
  NS_IMETHOD SendBlob(mozilla::dom::Blob *blob) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendBlob(blob); } \
  NS_IMETHOD Close(nsresult reason) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Close(reason); } 


#endif /* __gen_nsIPresentationSessionTransport_h__ */
