import React, { CSSProperties } from 'react';
import { PaginationModel, ParsedApiLogs } from '../LogTypes';

import ChevronLeftIcon from '@mui/icons-material/ChevronLeft';
import ChevronRightIcon from '@mui/icons-material/ChevronRight';
import { Button } from '@mui/material';

export interface PaginationSectionProps {
  pagination: PaginationModel;
  update: (p: PaginationModel) => void;
  results?: ParsedApiLogs;
}

const canMoveFoward = (logs?: ParsedApiLogs): boolean =>
  (logs?.total || 0) > (logs?.page || 0) * (logs?.page_size || 0);
const canMoveBack = (logs?: ParsedApiLogs): boolean => (logs?.page || 0) > 1;

export function PaginationSection(
  props: PaginationSectionProps
): React.ReactElement {
  const styles: { [k: string]: CSSProperties } = {
    pageOneOfManyLabel: {
      userSelect: 'none',
      msUserSelect: 'none',
      WebkitUserSelect: 'none',
    },
  };
  const goFoward = () => {
    props.update(
      Object.assign({}, props.pagination, {
        page: props.pagination.page + 1,
      })
    );
  };
  const goBack = () => {
    props.update(
      Object.assign({}, props.pagination, {
        page: props.pagination.page - 1,
      })
    );
  };
  return (
    <div>
      <Button onClick={() => goBack()} disabled={!canMoveBack(props.results)}>
        <ChevronLeftIcon />
      </Button>
      <Button
        onClick={() => goFoward()}
        disabled={!canMoveFoward(props.results)}
      >
        <ChevronRightIcon />
      </Button>
      {props.results && (canMoveFoward(props.results) || canMoveBack(props.results)) && (
        <span style={styles.pageOneOfManyLabel}>
          {props.results?.page} out of{' '}
          {Math.ceil(props.results?.total / props.results?.page_size)}
        </span>
      )}
    </div>
  );
}
