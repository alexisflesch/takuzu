import { Pipe, PipeTransform } from '@angular/core';


@Pipe({ name: 'TakuzuItemStyle' })
export class TakuzuItemStylePipe implements PipeTransform {
  transform(type: number): string {

    let classCSS: string;

    switch (type) {
      case 0:
        classCSS = 'empty';
        break;
      case 1:
        classCSS = 'empty';
        break;
      case 2:
        classCSS = 'two';
        break;
      case 3:
        classCSS = 'three';
        break;
      case 4:
        classCSS = 'empty';
        break;
      case 5:
        classCSS = 'empty';
        break;
      case 6:
        classCSS = 'empty';
        break;
      case 7:
        classCSS = "empty";
        break;
      default:
        classCSS = 'empty';
        break;
    }

    return classCSS;

  }
}