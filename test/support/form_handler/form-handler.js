export async function handleSubmit(formValues) {
  const formData = formValues.reduce((data, [key, [value]]) => {
    data[key] = value;
    return data;
  }, {});
  console.log('got called', formData);
  const httpResponse = await fetch('https://en266mx56nddc.x.pipedream.net/', {
    method: 'POST',
    headers: {
      'Accept': 'application/json',
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(formData)
  });
  console.log(httpResponse);
  return `Greetings, ${formData.name}!`;
}