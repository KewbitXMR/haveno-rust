// Core Models: [
//    Order(implements: ['PersistablePayload'], extends: [], requires: ['Product' || 'Service']])
// ]
// Requuiring Protobuf Message Types: [SignedOrder, MerchantSignedOrderUpdate, AccountSignedOrderUpdate, ProductInfo, OrderInfo, StoreInfo, Ack]
// Usage Intention by CoreModel: <Merchant> Applicable Functions: [sign_fullfilled(),sign_rejected(),sign_accepted()]
// Usage Intention by CoreModel: <Account> Application Functions: [sign(self|static)->SignedOrder|Exception]
// Generic Methods: [toProtobuf(self|static|class)->SignedOrder|Exception, fromProtobuf(self|static|class)->SignedOrder|Exception]