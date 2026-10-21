export class WidgetImageLoader {
  public async load(file: File): Promise<string> {
    if (
      file.size > 149_900 === true ||
      ['image/png', 'image/jpeg', 'image/webp'].includes(file.type) === false
    ) {
      throw new Error('Нужен PNG, JPEG или WebP до 150 КБ');
    }

    return new Promise((resolve, reject) => {
      const reader = new FileReader();

      reader.onload = (): void => {
        if (typeof reader.result === 'string') {
          resolve(reader.result);
        } else {
          reject(new Error('Не удалось прочитать изображение'));
        }
      };

      reader.onerror = (): void => {
        reject(new Error('Не удалось прочитать изображение'));
      };

      reader.readAsDataURL(file);
    });
  }
}
