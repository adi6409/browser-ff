/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIContentProcess.idl
 */

#ifndef __gen_nsIContentProcess_h__
#define __gen_nsIContentProcess_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */


/* starting interface:    nsIContentProcessInfo */
#define NS_ICONTENTPROCESSINFO_IID_STR "456f58be-29dd-4973-885b-95aece1c9a8a"

#define NS_ICONTENTPROCESSINFO_IID \
  {0x456f58be, 0x29dd, 0x4973, \
    { 0x88, 0x5b, 0x95, 0xae, 0xce, 0x1c, 0x9a, 0x8a }}

class NS_NO_VTABLE nsIContentProcessInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICONTENTPROCESSINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIContentProcessInfo;

  /* readonly attribute boolean isAlive; */
  NS_IMETHOD GetIsAlive(bool *aIsAlive) = 0;

  /* readonly attribute int32_t processId; */
  NS_IMETHOD GetProcessId(int32_t *aProcessId) = 0;

  /* readonly attribute int32_t tabCount; */
  NS_IMETHOD GetTabCount(int32_t *aTabCount) = 0;

  /* readonly attribute nsISupports messageManager; */
  NS_IMETHOD GetMessageManager(nsISupports **aMessageManager) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIContentProcessInfo, NS_ICONTENTPROCESSINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICONTENTPROCESSINFO \
  NS_IMETHOD GetIsAlive(bool *aIsAlive) override; \
  NS_IMETHOD GetProcessId(int32_t *aProcessId) override; \
  NS_IMETHOD GetTabCount(int32_t *aTabCount) override; \
  NS_IMETHOD GetMessageManager(nsISupports **aMessageManager) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICONTENTPROCESSINFO \
  nsresult GetIsAlive(bool *aIsAlive); \
  nsresult GetProcessId(int32_t *aProcessId); \
  nsresult GetTabCount(int32_t *aTabCount); \
  nsresult GetMessageManager(nsISupports **aMessageManager); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICONTENTPROCESSINFO(_to) \
  NS_IMETHOD GetIsAlive(bool *aIsAlive) override { return _to GetIsAlive(aIsAlive); } \
  NS_IMETHOD GetProcessId(int32_t *aProcessId) override { return _to GetProcessId(aProcessId); } \
  NS_IMETHOD GetTabCount(int32_t *aTabCount) override { return _to GetTabCount(aTabCount); } \
  NS_IMETHOD GetMessageManager(nsISupports **aMessageManager) override { return _to GetMessageManager(aMessageManager); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICONTENTPROCESSINFO(_to) \
  NS_IMETHOD GetIsAlive(bool *aIsAlive) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsAlive(aIsAlive); } \
  NS_IMETHOD GetProcessId(int32_t *aProcessId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProcessId(aProcessId); } \
  NS_IMETHOD GetTabCount(int32_t *aTabCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTabCount(aTabCount); } \
  NS_IMETHOD GetMessageManager(nsISupports **aMessageManager) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMessageManager(aMessageManager); } 


/* starting interface:    nsIContentProcessProvider */
#define NS_ICONTENTPROCESSPROVIDER_IID_STR "83ffb063-5f65-4c45-ae07-3f553e0809bb"

#define NS_ICONTENTPROCESSPROVIDER_IID \
  {0x83ffb063, 0x5f65, 0x4c45, \
    { 0xae, 0x07, 0x3f, 0x55, 0x3e, 0x08, 0x09, 0xbb }}

class NS_NO_VTABLE nsIContentProcessProvider : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICONTENTPROCESSPROVIDER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIContentProcessProvider;

  enum {
    NEW_PROCESS = -1
  };

  /* int32_t provideProcess (in AUTF8String aType, in Array<nsIContentProcessInfo> aAliveProcesses, in uint32_t aMaxCount); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ProvideProcess(const nsACString& aType, const nsTArray<RefPtr<nsIContentProcessInfo>>& aAliveProcesses, uint32_t aMaxCount, int32_t *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIContentProcessProvider, NS_ICONTENTPROCESSPROVIDER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICONTENTPROCESSPROVIDER \
  NS_IMETHOD ProvideProcess(const nsACString& aType, const nsTArray<RefPtr<nsIContentProcessInfo>>& aAliveProcesses, uint32_t aMaxCount, int32_t *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICONTENTPROCESSPROVIDER \
  nsresult ProvideProcess(const nsACString& aType, const nsTArray<RefPtr<nsIContentProcessInfo>>& aAliveProcesses, uint32_t aMaxCount, int32_t *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICONTENTPROCESSPROVIDER(_to) \
  NS_IMETHOD ProvideProcess(const nsACString& aType, const nsTArray<RefPtr<nsIContentProcessInfo>>& aAliveProcesses, uint32_t aMaxCount, int32_t *_retval) override { return _to ProvideProcess(aType, aAliveProcesses, aMaxCount, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICONTENTPROCESSPROVIDER(_to) \
  NS_IMETHOD ProvideProcess(const nsACString& aType, const nsTArray<RefPtr<nsIContentProcessInfo>>& aAliveProcesses, uint32_t aMaxCount, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ProvideProcess(aType, aAliveProcesses, aMaxCount, _retval); } 


#endif /* __gen_nsIContentProcess_h__ */
