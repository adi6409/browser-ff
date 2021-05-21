/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/layout/base/nsILayoutHistoryState.idl
 */

#ifndef __gen_nsILayoutHistoryState_h__
#define __gen_nsILayoutHistoryState_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include "nsStringFwd.h"
#include "mozilla/UniquePtr.h"
namespace mozilla {
class PresState;
} // namespace mozilla
template<typename> struct already_AddRefed;

/* starting interface:    nsILayoutHistoryState */
#define NS_ILAYOUTHISTORYSTATE_IID_STR "aef27cb3-4df9-4eeb-b0b0-ac56cf861d04"

#define NS_ILAYOUTHISTORYSTATE_IID \
  {0xaef27cb3, 0x4df9, 0x4eeb, \
    { 0xb0, 0xb0, 0xac, 0x56, 0xcf, 0x86, 0x1d, 0x04 }}

class NS_NO_VTABLE nsILayoutHistoryState : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILAYOUTHISTORYSTATE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsILayoutHistoryState;

  /* readonly attribute boolean hasStates; */
  NS_IMETHOD GetHasStates(bool *aHasStates) = 0;

  /* Array<ACString> getKeys (); */
  NS_IMETHOD GetKeys(nsTArray<nsCString >& _retval) = 0;

  /* void getPresState (in ACString aKey, out float aScrollX, out float aScrollY, out boolean aAllowScrollOriginDowngrade, out float aRes); */
  NS_IMETHOD GetPresState(const nsACString& aKey, float *aScrollX, float *aScrollY, bool *aAllowScrollOriginDowngrade, float *aRes) = 0;

  /* void addNewPresState (in ACString aKey, in float aScrollX, in float aScrollY, in boolean aAllowScrollOriginDowngrade, in float aRes); */
  NS_IMETHOD AddNewPresState(const nsACString& aKey, float aScrollX, float aScrollY, bool aAllowScrollOriginDowngrade, float aRes) = 0;

  /* [noscript,nostdcall,notxpcom] void AddState (in nsCString aKey, in PresStateUnique aState); */
  virtual void AddState(const nsCString & aKey, mozilla::UniquePtr<mozilla::PresState> aState) = 0;

  /* [noscript,nostdcall,notxpcom] PresStatePtr GetState (in nsCString aKey); */
  virtual mozilla::PresState * GetState(const nsCString & aKey) = 0;

  /* [noscript,nostdcall,notxpcom] void RemoveState (in nsCString aKey); */
  virtual void RemoveState(const nsCString & aKey) = 0;

  /* [noscript,nostdcall,notxpcom] boolean HasStates (); */
  virtual bool HasStates(void) = 0;

  /* [noscript,nostdcall,notxpcom] void SetScrollPositionOnly (in constBool aFlag); */
  virtual void SetScrollPositionOnly(const bool aFlag) = 0;

  /* [noscript,nostdcall,notxpcom] void ResetScrollState (); */
  virtual void ResetScrollState(void) = 0;

  /* [noscript,nostdcall,notxpcom] void GetContents (out boolean aScrollPositionOnly, out Array<ACString> aKeys, out Array<PresState> aStates); */
  virtual void GetContents(bool *aScrollPositionOnly, nsTArray<nsCString >& aKeys, nsTArray<mozilla::PresState >& aStates) = 0;

  /* [noscript,nostdcall,notxpcom] void Reset (); */
  virtual void Reset(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsILayoutHistoryState, NS_ILAYOUTHISTORYSTATE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILAYOUTHISTORYSTATE \
  NS_IMETHOD GetHasStates(bool *aHasStates) override; \
  NS_IMETHOD GetKeys(nsTArray<nsCString >& _retval) override; \
  NS_IMETHOD GetPresState(const nsACString& aKey, float *aScrollX, float *aScrollY, bool *aAllowScrollOriginDowngrade, float *aRes) override; \
  NS_IMETHOD AddNewPresState(const nsACString& aKey, float aScrollX, float aScrollY, bool aAllowScrollOriginDowngrade, float aRes) override; \
  virtual void AddState(const nsCString & aKey, mozilla::UniquePtr<mozilla::PresState> aState) override; \
  virtual mozilla::PresState * GetState(const nsCString & aKey) override; \
  virtual void RemoveState(const nsCString & aKey) override; \
  virtual bool HasStates(void) override; \
  virtual void SetScrollPositionOnly(const bool aFlag) override; \
  virtual void ResetScrollState(void) override; \
  virtual void GetContents(bool *aScrollPositionOnly, nsTArray<nsCString >& aKeys, nsTArray<mozilla::PresState >& aStates) override; \
  virtual void Reset(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILAYOUTHISTORYSTATE \
  nsresult GetHasStates(bool *aHasStates); \
  nsresult GetKeys(nsTArray<nsCString >& _retval); \
  nsresult GetPresState(const nsACString& aKey, float *aScrollX, float *aScrollY, bool *aAllowScrollOriginDowngrade, float *aRes); \
  nsresult AddNewPresState(const nsACString& aKey, float aScrollX, float aScrollY, bool aAllowScrollOriginDowngrade, float aRes); \
  void AddState(const nsCString & aKey, mozilla::UniquePtr<mozilla::PresState> aState); \
  mozilla::PresState * GetState(const nsCString & aKey); \
  void RemoveState(const nsCString & aKey); \
  bool HasStates(void); \
  void SetScrollPositionOnly(const bool aFlag); \
  void ResetScrollState(void); \
  void GetContents(bool *aScrollPositionOnly, nsTArray<nsCString >& aKeys, nsTArray<mozilla::PresState >& aStates); \
  void Reset(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILAYOUTHISTORYSTATE(_to) \
  NS_IMETHOD GetHasStates(bool *aHasStates) override { return _to GetHasStates(aHasStates); } \
  NS_IMETHOD GetKeys(nsTArray<nsCString >& _retval) override { return _to GetKeys(_retval); } \
  NS_IMETHOD GetPresState(const nsACString& aKey, float *aScrollX, float *aScrollY, bool *aAllowScrollOriginDowngrade, float *aRes) override { return _to GetPresState(aKey, aScrollX, aScrollY, aAllowScrollOriginDowngrade, aRes); } \
  NS_IMETHOD AddNewPresState(const nsACString& aKey, float aScrollX, float aScrollY, bool aAllowScrollOriginDowngrade, float aRes) override { return _to AddNewPresState(aKey, aScrollX, aScrollY, aAllowScrollOriginDowngrade, aRes); } \
  virtual void AddState(const nsCString & aKey, mozilla::UniquePtr<mozilla::PresState> aState) override { return _to AddState(aKey, aState); } \
  virtual mozilla::PresState * GetState(const nsCString & aKey) override { return _to GetState(aKey); } \
  virtual void RemoveState(const nsCString & aKey) override { return _to RemoveState(aKey); } \
  virtual bool HasStates(void) override { return _to HasStates(); } \
  virtual void SetScrollPositionOnly(const bool aFlag) override { return _to SetScrollPositionOnly(aFlag); } \
  virtual void ResetScrollState(void) override { return _to ResetScrollState(); } \
  virtual void GetContents(bool *aScrollPositionOnly, nsTArray<nsCString >& aKeys, nsTArray<mozilla::PresState >& aStates) override { return _to GetContents(aScrollPositionOnly, aKeys, aStates); } \
  virtual void Reset(void) override { return _to Reset(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILAYOUTHISTORYSTATE(_to) \
  NS_IMETHOD GetHasStates(bool *aHasStates) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHasStates(aHasStates); } \
  NS_IMETHOD GetKeys(nsTArray<nsCString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetKeys(_retval); } \
  NS_IMETHOD GetPresState(const nsACString& aKey, float *aScrollX, float *aScrollY, bool *aAllowScrollOriginDowngrade, float *aRes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPresState(aKey, aScrollX, aScrollY, aAllowScrollOriginDowngrade, aRes); } \
  NS_IMETHOD AddNewPresState(const nsACString& aKey, float aScrollX, float aScrollY, bool aAllowScrollOriginDowngrade, float aRes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddNewPresState(aKey, aScrollX, aScrollY, aAllowScrollOriginDowngrade, aRes); } \
  virtual void AddState(const nsCString & aKey, mozilla::UniquePtr<mozilla::PresState> aState) override; \
  virtual mozilla::PresState * GetState(const nsCString & aKey) override; \
  virtual void RemoveState(const nsCString & aKey) override; \
  virtual bool HasStates(void) override; \
  virtual void SetScrollPositionOnly(const bool aFlag) override; \
  virtual void ResetScrollState(void) override; \
  virtual void GetContents(bool *aScrollPositionOnly, nsTArray<nsCString >& aKeys, nsTArray<mozilla::PresState >& aStates) override; \
  virtual void Reset(void) override; 

/* Defined in nsLayoutHistoryState.cpp */
already_AddRefed<nsILayoutHistoryState>
NS_NewLayoutHistoryState();
namespace mozilla {
mozilla::UniquePtr<mozilla::PresState> NewPresState();
} // namespace mozilla

#endif /* __gen_nsILayoutHistoryState_h__ */
