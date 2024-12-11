export class Note
{
  title: string = '';
  text: string = '';
  createdAt : Date = new Date();

  constructor() {
    this.text = "foo";
    this.title = "bar";
    this.createdAt = new Date();
  }
}
