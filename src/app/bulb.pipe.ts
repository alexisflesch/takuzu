import { Pipe, PipeTransform } from '@angular/core';

interface gameStatus {
  nbClues: number;
  completed: boolean;
  solved: boolean;
}

@Pipe({
  name: 'bulb'
})
export class BulbPipe implements PipeTransform {

  transform(type: gameStatus): string {

    let classCSS: string = 'mat-icon-bulb';

    if (type['nbClues'] == 0 && !type['completed']) {
      classCSS = 'mat-icon-disabled'
    }
    else if (type['completed'] && !type['solved']) {
      classCSS = 'mat-icon-blink'
    }
    return classCSS
  }


}
