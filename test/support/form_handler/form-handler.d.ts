/**
* # Variants
* 
* ## `"success"`
* 
* ## `"error"`
*/
export type Outcome = 'success' | 'error';
export type FormValue = [string, Array<string>];
export type FormError = [number, string];
export type FormResult = [Outcome, string];
export function handleSubmit(formData: Array<FormValue>): FormResult;
