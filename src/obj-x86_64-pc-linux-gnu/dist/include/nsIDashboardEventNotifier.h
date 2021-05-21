/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIDashboardEventNotifier.idl
 */

#ifndef __gen_nsIDashboardEventNotifier_h__
#define __gen_nsIDashboardEventNotifier_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIDashboardEventNotifier */
#define NS_IDASHBOARDEVENTNOTIFIER_IID_STR "24fdfcbe-54cb-4997-8392-3c476126ea3b"

#define NS_IDASHBOARDEVENTNOTIFIER_IID \
  {0x24fdfcbe, 0x54cb, 0x4997, \
    { 0x83, 0x92, 0x3c, 0x47, 0x61, 0x26, 0xea, 0x3b }}

class NS_NO_VTABLE nsIDashboardEventNotifier : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDASHBOARDEVENTNOTIFIER_IID)

  /* void addHost (in ACString aHost, in unsigned long aSerial, in boolean aEncrypted); */
  NS_IMETHOD AddHost(const nsACString& aHost, uint32_t aSerial, bool aEncrypted) = 0;

  /* void removeHost (in ACString aHost, in unsigned long aSerial); */
  NS_IMETHOD RemoveHost(const nsACString& aHost, uint32_t aSerial) = 0;

  /* void newMsgSent (in ACString aHost, in unsigned long aSerial, in unsigned long aLength); */
  NS_IMETHOD NewMsgSent(const nsACString& aHost, uint32_t aSerial, uint32_t aLength) = 0;

  /* void newMsgReceived (in ACString aHost, in unsigned long aSerial, in unsigned long aLength); */
  NS_IMETHOD NewMsgReceived(const nsACString& aHost, uint32_t aSerial, uint32_t aLength) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDashboardEventNotifier, NS_IDASHBOARDEVENTNOTIFIER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDASHBOARDEVENTNOTIFIER \
  NS_IMETHOD AddHost(const nsACString& aHost, uint32_t aSerial, bool aEncrypted) override; \
  NS_IMETHOD RemoveHost(const nsACString& aHost, uint32_t aSerial) override; \
  NS_IMETHOD NewMsgSent(const nsACString& aHost, uint32_t aSerial, uint32_t aLength) override; \
  NS_IMETHOD NewMsgReceived(const nsACString& aHost, uint32_t aSerial, uint32_t aLength) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDASHBOARDEVENTNOTIFIER \
  nsresult AddHost(const nsACString& aHost, uint32_t aSerial, bool aEncrypted); \
  nsresult RemoveHost(const nsACString& aHost, uint32_t aSerial); \
  nsresult NewMsgSent(const nsACString& aHost, uint32_t aSerial, uint32_t aLength); \
  nsresult NewMsgReceived(const nsACString& aHost, uint32_t aSerial, uint32_t aLength); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDASHBOARDEVENTNOTIFIER(_to) \
  NS_IMETHOD AddHost(const nsACString& aHost, uint32_t aSerial, bool aEncrypted) override { return _to AddHost(aHost, aSerial, aEncrypted); } \
  NS_IMETHOD RemoveHost(const nsACString& aHost, uint32_t aSerial) override { return _to RemoveHost(aHost, aSerial); } \
  NS_IMETHOD NewMsgSent(const nsACString& aHost, uint32_t aSerial, uint32_t aLength) override { return _to NewMsgSent(aHost, aSerial, aLength); } \
  NS_IMETHOD NewMsgReceived(const nsACString& aHost, uint32_t aSerial, uint32_t aLength) override { return _to NewMsgReceived(aHost, aSerial, aLength); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDASHBOARDEVENTNOTIFIER(_to) \
  NS_IMETHOD AddHost(const nsACString& aHost, uint32_t aSerial, bool aEncrypted) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddHost(aHost, aSerial, aEncrypted); } \
  NS_IMETHOD RemoveHost(const nsACString& aHost, uint32_t aSerial) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveHost(aHost, aSerial); } \
  NS_IMETHOD NewMsgSent(const nsACString& aHost, uint32_t aSerial, uint32_t aLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewMsgSent(aHost, aSerial, aLength); } \
  NS_IMETHOD NewMsgReceived(const nsACString& aHost, uint32_t aSerial, uint32_t aLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewMsgReceived(aHost, aSerial, aLength); } 


#endif /* __gen_nsIDashboardEventNotifier_h__ */
