export class Note
{
  id: bigint;
  userId: string;
  title: string = '';
  text: string = '';
  createdAt : Date = new Date();

  constructor(id: bigint) {
    this.id = id;
    this.userId = "";
    this.text = "foo";
    this.title = "bar";
    this.createdAt = new Date();
  }

  toJSON() {
    return {
      id: this.id.toString(),
      userId: this.userId,
      title: this.title,
      text: this.text,
      createdAt: this.createdAt.toISOString(), 
    }
  }
}
