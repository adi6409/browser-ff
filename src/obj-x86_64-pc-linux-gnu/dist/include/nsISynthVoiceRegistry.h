/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/webspeech/synth/nsISynthVoiceRegistry.idl
 */

#ifndef __gen_nsISynthVoiceRegistry_h__
#define __gen_nsISynthVoiceRegistry_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsISpeechService; /* forward declaration */


/* starting interface:    nsISynthVoiceRegistry */
#define NS_ISYNTHVOICEREGISTRY_IID_STR "5d7a0b38-77e5-4ee5-897c-ce5db9b85d44"

#define NS_ISYNTHVOICEREGISTRY_IID \
  {0x5d7a0b38, 0x77e5, 0x4ee5, \
    { 0x89, 0x7c, 0xce, 0x5d, 0xb9, 0xb8, 0x5d, 0x44 }}

class NS_NO_VTABLE nsISynthVoiceRegistry : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISYNTHVOICEREGISTRY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISynthVoiceRegistry;

  /* void addVoice (in nsISpeechService aService, in AString aUri, in AString aName, in AString aLang, in boolean aLocalService, in boolean aQueuesUtterances); */
  NS_IMETHOD AddVoice(nsISpeechService *aService, const nsAString& aUri, const nsAString& aName, const nsAString& aLang, bool aLocalService, bool aQueuesUtterances) = 0;

  /* void removeVoice (in nsISpeechService aService, in AString aUri); */
  NS_IMETHOD RemoveVoice(nsISpeechService *aService, const nsAString& aUri) = 0;

  /* void notifyVoicesChanged (); */
  NS_IMETHOD NotifyVoicesChanged(void) = 0;

  /* void setDefaultVoice (in AString aUri, in boolean aIsDefault); */
  NS_IMETHOD SetDefaultVoice(const nsAString& aUri, bool aIsDefault) = 0;

  /* readonly attribute uint32_t voiceCount; */
  NS_IMETHOD GetVoiceCount(uint32_t *aVoiceCount) = 0;

  /* AString getVoice (in uint32_t aIndex); */
  NS_IMETHOD GetVoice(uint32_t aIndex, nsAString& _retval) = 0;

  /* bool isDefaultVoice (in AString aUri); */
  NS_IMETHOD IsDefaultVoice(const nsAString& aUri, bool *_retval) = 0;

  /* bool isLocalVoice (in AString aUri); */
  NS_IMETHOD IsLocalVoice(const nsAString& aUri, bool *_retval) = 0;

  /* AString getVoiceLang (in AString aUri); */
  NS_IMETHOD GetVoiceLang(const nsAString& aUri, nsAString& _retval) = 0;

  /* AString getVoiceName (in AString aUri); */
  NS_IMETHOD GetVoiceName(const nsAString& aUri, nsAString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISynthVoiceRegistry, NS_ISYNTHVOICEREGISTRY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISYNTHVOICEREGISTRY \
  NS_IMETHOD AddVoice(nsISpeechService *aService, const nsAString& aUri, const nsAString& aName, const nsAString& aLang, bool aLocalService, bool aQueuesUtterances) override; \
  NS_IMETHOD RemoveVoice(nsISpeechService *aService, const nsAString& aUri) override; \
  NS_IMETHOD NotifyVoicesChanged(void) override; \
  NS_IMETHOD SetDefaultVoice(const nsAString& aUri, bool aIsDefault) override; \
  NS_IMETHOD GetVoiceCount(uint32_t *aVoiceCount) override; \
  NS_IMETHOD GetVoice(uint32_t aIndex, nsAString& _retval) override; \
  NS_IMETHOD IsDefaultVoice(const nsAString& aUri, bool *_retval) override; \
  NS_IMETHOD IsLocalVoice(const nsAString& aUri, bool *_retval) override; \
  NS_IMETHOD GetVoiceLang(const nsAString& aUri, nsAString& _retval) override; \
  NS_IMETHOD GetVoiceName(const nsAString& aUri, nsAString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISYNTHVOICEREGISTRY \
  nsresult AddVoice(nsISpeechService *aService, const nsAString& aUri, const nsAString& aName, const nsAString& aLang, bool aLocalService, bool aQueuesUtterances); \
  nsresult RemoveVoice(nsISpeechService *aService, const nsAString& aUri); \
  nsresult NotifyVoicesChanged(void); \
  nsresult SetDefaultVoice(const nsAString& aUri, bool aIsDefault); \
  nsresult GetVoiceCount(uint32_t *aVoiceCount); \
  nsresult GetVoice(uint32_t aIndex, nsAString& _retval); \
  nsresult IsDefaultVoice(const nsAString& aUri, bool *_retval); \
  nsresult IsLocalVoice(const nsAString& aUri, bool *_retval); \
  nsresult GetVoiceLang(const nsAString& aUri, nsAString& _retval); \
  nsresult GetVoiceName(const nsAString& aUri, nsAString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISYNTHVOICEREGISTRY(_to) \
  NS_IMETHOD AddVoice(nsISpeechService *aService, const nsAString& aUri, const nsAString& aName, const nsAString& aLang, bool aLocalService, bool aQueuesUtterances) override { return _to AddVoice(aService, aUri, aName, aLang, aLocalService, aQueuesUtterances); } \
  NS_IMETHOD RemoveVoice(nsISpeechService *aService, const nsAString& aUri) override { return _to RemoveVoice(aService, aUri); } \
  NS_IMETHOD NotifyVoicesChanged(void) override { return _to NotifyVoicesChanged(); } \
  NS_IMETHOD SetDefaultVoice(const nsAString& aUri, bool aIsDefault) override { return _to SetDefaultVoice(aUri, aIsDefault); } \
  NS_IMETHOD GetVoiceCount(uint32_t *aVoiceCount) override { return _to GetVoiceCount(aVoiceCount); } \
  NS_IMETHOD GetVoice(uint32_t aIndex, nsAString& _retval) override { return _to GetVoice(aIndex, _retval); } \
  NS_IMETHOD IsDefaultVoice(const nsAString& aUri, bool *_retval) override { return _to IsDefaultVoice(aUri, _retval); } \
  NS_IMETHOD IsLocalVoice(const nsAString& aUri, bool *_retval) override { return _to IsLocalVoice(aUri, _retval); } \
  NS_IMETHOD GetVoiceLang(const nsAString& aUri, nsAString& _retval) override { return _to GetVoiceLang(aUri, _retval); } \
  NS_IMETHOD GetVoiceName(const nsAString& aUri, nsAString& _retval) override { return _to GetVoiceName(aUri, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISYNTHVOICEREGISTRY(_to) \
  NS_IMETHOD AddVoice(nsISpeechService *aService, const nsAString& aUri, const nsAString& aName, const nsAString& aLang, bool aLocalService, bool aQueuesUtterances) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddVoice(aService, aUri, aName, aLang, aLocalService, aQueuesUtterances); } \
  NS_IMETHOD RemoveVoice(nsISpeechService *aService, const nsAString& aUri) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveVoice(aService, aUri); } \
  NS_IMETHOD NotifyVoicesChanged(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyVoicesChanged(); } \
  NS_IMETHOD SetDefaultVoice(const nsAString& aUri, bool aIsDefault) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDefaultVoice(aUri, aIsDefault); } \
  NS_IMETHOD GetVoiceCount(uint32_t *aVoiceCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetVoiceCount(aVoiceCount); } \
  NS_IMETHOD GetVoice(uint32_t aIndex, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetVoice(aIndex, _retval); } \
  NS_IMETHOD IsDefaultVoice(const nsAString& aUri, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsDefaultVoice(aUri, _retval); } \
  NS_IMETHOD IsLocalVoice(const nsAString& aUri, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsLocalVoice(aUri, _retval); } \
  NS_IMETHOD GetVoiceLang(const nsAString& aUri, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetVoiceLang(aUri, _retval); } \
  NS_IMETHOD GetVoiceName(const nsAString& aUri, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetVoiceName(aUri, _retval); } 

#define NS_SYNTHVOICEREGISTRY_CID                   \
  { /* {7090524d-5574-4492-a77f-d8d558ced59d} */       \
    0x7090524d,                                        \
    0x5574,                                            \
    0x4492,                                            \
    { 0xa7, 0x7f, 0xd8, 0xd5, 0x58, 0xce, 0xd5, 0x9d } \
  }
#define NS_SYNTHVOICEREGISTRY_CONTRACTID \
    "@mozilla.org/synth-voice-registry;1"
#define NS_SYNTHVOICEREGISTRY_CLASSNAME \
    "Speech Synthesis Voice Registry"

#endif /* __gen_nsISynthVoiceRegistry_h__ */
