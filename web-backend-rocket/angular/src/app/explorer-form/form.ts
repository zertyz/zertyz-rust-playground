
export interface Form {
  service: string,
  method: HttpMethod | string,
  form_title: string,
  service_description: string,
  fields: FormField[][],
}

export interface FormField {
  label: string,
  name: string,
  dataType: FieldDataType | string,
  presentationType: FieldPresentationType | string,
  required?: boolean,
  helperText: string,
  possibleValues?: {text: string, value: string}[],
  possibleRangeStart?: number,
  possibleRangeFinish?: number,
}

export enum FieldDataType {
  Integer = "Integer",
  Float = "Float",
  Text = "Text",
  Boolean = "Boolean",
}

export enum FieldPresentationType {
  Input = "Input",
  SteppedInput = "SteppedInput",
  Area = "Area",
  Date = "Date",
  DateTime = "DateTime",
  Toggle = "Toggle",
  ComboBox = "ComboBox",
  ListBox = "ListBox",
  CheckBoxes = "CheckBoxes",
  RadioButtons = "RadioButtons",
}

export enum HttpMethod {
  GET = "GET",
  POST = "POST",
}
