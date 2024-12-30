export class Note
{
  id: bigint;
  title: string = '';
  text: string = '';
  createdAt : Date = new Date();

  constructor(id: bigint) {
    this.id = id;
    this.text = "foo";
    this.title = "bar";
    this.createdAt = new Date();
  }

  toJSON() {
    return {
      id: this.id.toString(),
      title: this.title,
      text: this.text,
      createdAt: this.createdAt, 
    }
  }
}
