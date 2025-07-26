export class TableEntry {
  key: string;
  value: string;
  description: string;

  constructor(
    key: string = '',
    value: string = '',
    description: string = ''
  ) {
    this.key = key;
    this.value = value;
    this.description = description;
  }
}
