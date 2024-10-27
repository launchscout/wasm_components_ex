export function init() {
  return {
    customers: [
      {
        firstName: "bob",
        lastName: "jones",
        email: "bob@jones.com"
      }
    ],
    editingCustomer: {
      firstName: "bob",
      lastName: "notbob",
      email: "bob@notbob.com"
    }
  };
}

export function addCustomer(customer, state) {
  return state;
}