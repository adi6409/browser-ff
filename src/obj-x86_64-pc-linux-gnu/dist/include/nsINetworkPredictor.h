/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsINetworkPredictor.idl
 */

#ifndef __gen_nsINetworkPredictor_h__
#define __gen_nsINetworkPredictor_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsINetworkPredictorVerifier; /* forward declaration */

namespace mozilla {
namespace dom {
class Document; /* webidl Document */
} // namespace dom
} // namespace mozilla

typedef uint32_t  PredictorPredictReason;

typedef uint32_t  PredictorLearnReason;

namespace mozilla {
class OriginAttributes;
}

/* starting interface:    nsINetworkPredictor */
#define NS_INETWORKPREDICTOR_IID_STR "acc88e7c-3f39-42c7-ac31-6377c2c3d73e"

#define NS_INETWORKPREDICTOR_IID \
  {0xacc88e7c, 0x3f39, 0x42c7, \
    { 0xac, 0x31, 0x63, 0x77, 0xc2, 0xc3, 0xd7, 0x3e }}

class NS_NO_VTABLE nsINetworkPredictor : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INETWORKPREDICTOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINetworkPredictor;

  enum {
    PREDICT_LINK = 0U,
    PREDICT_LOAD = 1U,
    PREDICT_STARTUP = 2U
  };

  /* [implicit_jscontext] void predict (in nsIURI targetURI, in nsIURI sourceURI, in PredictorPredictReason reason, in jsval originAttributes, in nsINetworkPredictorVerifier verifier); */
  NS_IMETHOD Predict(nsIURI *targetURI, nsIURI *sourceURI, PredictorPredictReason reason, JS::HandleValue originAttributes, nsINetworkPredictorVerifier *verifier, JSContext* cx) = 0;

  /* [notxpcom] nsresult predictNative (in nsIURI targetURI, in nsIURI sourceURI, in PredictorPredictReason reason, in OriginAttributes originAttributes, in nsINetworkPredictorVerifier verifier); */
  NS_IMETHOD PredictNative(nsIURI *targetURI, nsIURI *sourceURI, PredictorPredictReason reason, const mozilla::OriginAttributes & originAttributes, nsINetworkPredictorVerifier *verifier) = 0;

  enum {
    LEARN_LOAD_TOPLEVEL = 0U,
    LEARN_LOAD_SUBRESOURCE = 1U,
    LEARN_LOAD_REDIRECT = 2U,
    LEARN_STARTUP = 3U
  };

  /* [implicit_jscontext] void learn (in nsIURI targetURI, in nsIURI sourceURI, in PredictorLearnReason reason, in jsval originAttributes); */
  NS_IMETHOD Learn(nsIURI *targetURI, nsIURI *sourceURI, PredictorLearnReason reason, JS::HandleValue originAttributes, JSContext* cx) = 0;

  /* [notxpcom] nsresult learnNative (in nsIURI targetURI, in nsIURI sourceURI, in PredictorLearnReason reason, in OriginAttributes originAttributes); */
  NS_IMETHOD LearnNative(nsIURI *targetURI, nsIURI *sourceURI, PredictorLearnReason reason, const mozilla::OriginAttributes & originAttributes) = 0;

  /* void reset (); */
  NS_IMETHOD Reset(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINetworkPredictor, NS_INETWORKPREDICTOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINETWORKPREDICTOR \
  NS_IMETHOD Predict(nsIURI *targetURI, nsIURI *sourceURI, PredictorPredictReason reason, JS::HandleValue originAttributes, nsINetworkPredictorVerifier *verifier, JSContext* cx) override; \
  NS_IMETHOD PredictNative(nsIURI *targetURI, nsIURI *sourceURI, PredictorPredictReason reason, const mozilla::OriginAttributes & originAttributes, nsINetworkPredictorVerifier *verifier) override; \
  NS_IMETHOD Learn(nsIURI *targetURI, nsIURI *sourceURI, PredictorLearnReason reason, JS::HandleValue originAttributes, JSContext* cx) override; \
  NS_IMETHOD LearnNative(nsIURI *targetURI, nsIURI *sourceURI, PredictorLearnReason reason, const mozilla::OriginAttributes & originAttributes) override; \
  NS_IMETHOD Reset(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINETWORKPREDICTOR \
  nsresult Predict(nsIURI *targetURI, nsIURI *sourceURI, PredictorPredictReason reason, JS::HandleValue originAttributes, nsINetworkPredictorVerifier *verifier, JSContext* cx); \
  nsresult PredictNative(nsIURI *targetURI, nsIURI *sourceURI, PredictorPredictReason reason, const mozilla::OriginAttributes & originAttributes, nsINetworkPredictorVerifier *verifier); \
  nsresult Learn(nsIURI *targetURI, nsIURI *sourceURI, PredictorLearnReason reason, JS::HandleValue originAttributes, JSContext* cx); \
  nsresult LearnNative(nsIURI *targetURI, nsIURI *sourceURI, PredictorLearnReason reason, const mozilla::OriginAttributes & originAttributes); \
  nsresult Reset(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINETWORKPREDICTOR(_to) \
  NS_IMETHOD Predict(nsIURI *targetURI, nsIURI *sourceURI, PredictorPredictReason reason, JS::HandleValue originAttributes, nsINetworkPredictorVerifier *verifier, JSContext* cx) override { return _to Predict(targetURI, sourceURI, reason, originAttributes, verifier, cx); } \
  NS_IMETHOD PredictNative(nsIURI *targetURI, nsIURI *sourceURI, PredictorPredictReason reason, const mozilla::OriginAttributes & originAttributes, nsINetworkPredictorVerifier *verifier) override { return _to PredictNative(targetURI, sourceURI, reason, originAttributes, verifier); } \
  NS_IMETHOD Learn(nsIURI *targetURI, nsIURI *sourceURI, PredictorLearnReason reason, JS::HandleValue originAttributes, JSContext* cx) override { return _to Learn(targetURI, sourceURI, reason, originAttributes, cx); } \
  NS_IMETHOD LearnNative(nsIURI *targetURI, nsIURI *sourceURI, PredictorLearnReason reason, const mozilla::OriginAttributes & originAttributes) override { return _to LearnNative(targetURI, sourceURI, reason, originAttributes); } \
  NS_IMETHOD Reset(void) override { return _to Reset(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINETWORKPREDICTOR(_to) \
  NS_IMETHOD Predict(nsIURI *targetURI, nsIURI *sourceURI, PredictorPredictReason reason, JS::HandleValue originAttributes, nsINetworkPredictorVerifier *verifier, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Predict(targetURI, sourceURI, reason, originAttributes, verifier, cx); } \
  NS_IMETHOD PredictNative(nsIURI *targetURI, nsIURI *sourceURI, PredictorPredictReason reason, const mozilla::OriginAttributes & originAttributes, nsINetworkPredictorVerifier *verifier) override; \
  NS_IMETHOD Learn(nsIURI *targetURI, nsIURI *sourceURI, PredictorLearnReason reason, JS::HandleValue originAttributes, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Learn(targetURI, sourceURI, reason, originAttributes, cx); } \
  NS_IMETHOD LearnNative(nsIURI *targetURI, nsIURI *sourceURI, PredictorLearnReason reason, const mozilla::OriginAttributes & originAttributes) override; \
  NS_IMETHOD Reset(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Reset(); } 

// Wrapper functions to make use of the predictor easier and less invasive
class nsIChannel;
class nsILoadContext;
class nsILoadGroup;
class nsINetworkPredictorVerifier;
namespace mozilla {
class OriginAttributes;
namespace net {
nsresult PredictorPredict(nsIURI *targetURI,
                          nsIURI *sourceURI,
                          PredictorPredictReason reason,
                          const OriginAttributes& originAttributes,
                          nsINetworkPredictorVerifier *verifier);
nsresult PredictorLearn(nsIURI *targetURI,
                        nsIURI *sourceURI,
                        PredictorLearnReason reason,
                        const OriginAttributes& originAttributes);
nsresult PredictorLearn(nsIURI *targetURI,
                        nsIURI *sourceURI,
                        PredictorLearnReason reason,
                        nsILoadGroup *loadGroup);
nsresult PredictorLearn(nsIURI *targetURI,
                        nsIURI *sourceURI,
                        PredictorLearnReason reason,
                        dom::Document *document);
nsresult PredictorLearnRedirect(nsIURI *targetURI,
                                nsIChannel *channel,
                                const OriginAttributes& originAttributes);
} // mozilla::net
} // mozilla

#endif /* __gen_nsINetworkPredictor_h__ */
